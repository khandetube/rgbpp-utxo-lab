use bitcoin::consensus::{deserialize, serialize};
use bitcoin::hashes::{sha256d, Hash};
use bitcoin::pow::{CompactTarget, Target};
use bitcoin::{
    absolute, block, Amount, Block, BlockHash, ScriptBuf, Sequence, Transaction, TxIn, TxMerkleNode,
    TxOut, Txid, Witness,
};
use bitcoin::pow::{CompactTarget, Target};
use serde_json::Value;
use std::error::Error;

pub type BoxError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct BlockTemplate {
    pub height: u64,
    pub version: block::Version,
    pub previous_blockhash: BlockHash,
    pub bits: CompactTarget,
    pub target: Target,
    pub curtime: u32,
    pub mintime: u32,
    pub transactions: Vec<Transaction>,
    pub coinbase_value: u64,
    pub default_witness_commitment: Option<ScriptBuf>,
}

impl BlockTemplate {
    pub fn from_json(value: &Value) -> Result<Self, BoxError> {
        let height = value["height"].as_u64().ok_or("missing height")?;
        let version = value["version"].as_i64().ok_or("missing version")? as i32;
        let previous_blockhash: BlockHash = value["previousblockhash"]
            .as_str()
            .ok_or("missing previousblockhash")?
            .parse()?;
        let bits_hex = value["bits"].as_str().ok_or("missing bits")?;
        let bits = CompactTarget::from_consensus(u32::from_str_radix(bits_hex, 16)?);
        let target = Target::from_compact(bits);
        let target_json = value["target"].as_str().ok_or("missing target")?;
        if target_json.len() != 64 { return Err("target must be 32 bytes of hex".into()); }
        let parsed_target = bitcoin::uint::Uint256::from_str_radix(target_json, 16).map_err(|_| "invalid target")?;
        if parsed_target != target.to_uint256() { return Err("bits/target mismatch".into()); }
        let curtime = value["curtime"].as_u64().ok_or("missing curtime")? as u32;
        let mintime = value["mintime"].as_u64().ok_or("missing mintime")? as u32;
        let coinbase_value = value["coinbasevalue"].as_u64().unwrap_or(0);

        let transactions = value["transactions"].as_array().ok_or("missing transactions")?
            .iter().map(|tx| {
                let data = tx["data"].as_str().ok_or("template transaction missing data")?;
                Ok(deserialize(&hex::decode(data)?)?)
            }).collect::<Result<Vec<Transaction>, BoxError>>()?;

        let default_witness_commitment = value["default_witness_commitment"]
            .as_str().map(|s| Ok(ScriptBuf::from_bytes(hex::decode(s)?)))
            .transpose()?;

        Ok(Self {
            height,
            version: block::Version::from_consensus(version),
            previous_blockhash,
            bits,
            target,
            curtime,
            mintime,
            transactions,
            coinbase_value,
            default_witness_commitment,
        })
    }

    pub fn coinbase(&self, extra_nonce: u64, payout_script: Option<&ScriptBuf>) -> Transaction {
        let tag = extra_nonce.to_le_bytes();
        let script_sig = bitcoin::script::Builder::new()
            .push_int(self.height as i64)
            .push_slice(b"rgbpp-utxo-lab")
            .push_slice(tag)
            .into_script();

        let mut outputs = Vec::new();
        if let Some(script) = payout_script {
            outputs.push(TxOut { value: Amount::from_sat(self.coinbase_value), script_pubkey: script.clone() });
        } else {
            outputs.push(TxOut { value: Amount::from_sat(0), script_pubkey: ScriptBuf::new_op_return(b"rgbpp-utxo-lab") });
        }
        if let Some(commitment) = &self.default_witness_commitment {
            outputs.push(TxOut { value: Amount::from_sat(0), script_pubkey: commitment.clone() });
        }

        let witness = if self.default_witness_commitment.is_some() {
            Witness::from_slice(&[vec![0u8; 32]])
        } else {
            Witness::default()
        };

        Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: bitcoin::OutPoint::null(),
                script_sig: ScriptBuf::from_bytes(script_sig),
                sequence: Sequence::MAX,
                witness,
            }],
            output: outputs,
        }
    }

    pub fn build_block(
        &self,
        extra_nonce: u64,
        time: u32,
        nonce: u32,
        payout_script: Option<&ScriptBuf>,
    ) -> Block {
        let coinbase = self.coinbase(extra_nonce, payout_script);
        let mut txs = Vec::with_capacity(self.transactions.len() + 1);
        txs.push(coinbase);
        txs.extend(self.transactions.iter().cloned());
        let merkle_root = merkle_root(txs.iter().map(Transaction::compute_txid));
        Block {
            header: block::Header {
                version: self.version,
                prev_blockhash: self.previous_blockhash,
                merkle_root,
                time,
                bits: self.bits,
                nonce,
            },
            txdata: txs,
        }
    }
}

pub fn merkle_root<I>(txids: I) -> TxMerkleNode
where
    I: IntoIterator<Item = Txid>,
{
    let mut level: Vec<[u8; 32]> = txids.into_iter().map(|x| x.to_byte_array()).collect();
    if level.is_empty() {
        return TxMerkleNode::all_zeros();
    }
    while level.len() > 1 {
        let mut next = Vec::with_capacity((level.len() + 1) / 2);
        let mut i = 0;
        while i < level.len() {
            let right = if i + 1 < level.len() { level[i + 1] } else { level[i] };
            let mut bytes = [0u8; 64];
            bytes[..32].copy_from_slice(&level[i]);
            bytes[32..].copy_from_slice(&right);
            next.push(sha256d::Hash::hash(&bytes).to_byte_array());
            i += 2;
        }
        level = next;
    }
    TxMerkleNode::from_byte_array(level[0])
}

pub fn serialize_block(block: &Block) -> String {
    hex::encode(serialize(block))
}

pub fn verify_candidate(block: &Block, target: Target) -> bool {
    target.is_met_by(block.header.block_hash())
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_and_builds_a_minimal_template() {
        let value = json!({
            "height": 200,
            "version": 1,
            "previousblockhash": "0000000000000000000000000000000000000000000000000000000000000000",
            "bits": "207fffff",
            "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
            "curtime": 1700000000u64,
            "mintime": 1699999900u64,
            "coinbasevalue": 5000000000u64,
            "transactions": []
        });
        let template = BlockTemplate::from_json(&value).unwrap();
        let block = template.build_block(7, template.curtime, 0, None);
        assert_eq!(block.txdata.len(), 1);
        assert!(block.txdata[0].is_coinbase());
        assert_eq!(block.header.prev_blockhash, template.previous_blockhash);
        assert_eq!(block.header.merkle_root, block.txdata[0].compute_txid().into());
        assert!(verify_candidate(&block, template.target));
    }

    #[test]
    fn witness_commitment_adds_reserved_value() {
        let value = json!({
            "height": 201,
            "version": 1,
            "previousblockhash": "0000000000000000000000000000000000000000000000000000000000000000",
            "bits": "207fffff",
            "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
            "curtime": 1700000000u64,
            "mintime": 1699999900u64,
            "coinbasevalue": 5000000000u64,
            "default_witness_commitment": "6a24aa21a9ed00000000000000000000000000000000000000000000000000000000000000",
            "transactions": []
        });
        let template = BlockTemplate::from_json(&value).unwrap();
        let coinbase = template.coinbase(1, None);
        assert_eq!(coinbase.input[0].witness.len(), 1);
        assert_eq!(coinbase.input[0].witness.first().unwrap().len(), 32);
        assert!(coinbase.output.iter().any(|o| o.script_pubkey.to_bytes().starts_with(&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed])));
    }
}
