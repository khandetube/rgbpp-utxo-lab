use std::{collections::HashMap, env, error::Error, str::FromStr};

use anyhow::{anyhow, Context, Result};
use ckb_hash::blake2b_256;
use ckb_jsonrpc_types::{OutputsValidator, TransactionView as JsonTransactionView};
use ckb_sdk::{
    constants::SIGHASH_TYPE_HASH,
    rpc::CkbRpcClient,
    traits::{
        DefaultCellCollector, DefaultCellDepResolver, DefaultHeaderDepResolver,
        DefaultTransactionDependencyProvider, SecpCkbRawKeySigner,
    },
    tx_builder::{transfer::CapacityTransferBuilder, CapacityBalancer, TxBuilder},
    unlock::{ScriptUnlocker, SecpSighashUnlocker},
    Address, HumanCapacity, ScriptId, SECP256K1,
};
use ckb_types::{
    bytes::Bytes,
    core::{BlockView, ScriptHashType},
    packed::{CellOutput, Script, WitnessArgs},
    prelude::*,
    H256,
};

pub struct TransferConfig {
    pub rpc_url: String,
    pub sender_key: H256,
    pub receiver: Address,
    pub capacity: HumanCapacity,
    pub fee_rate: u64,
    pub broadcast: bool,
}

pub fn build_and_optionally_send(config: TransferConfig) -> Result<H256> {
    validate_broadcast_network(&config)?;
    let sender_key = secp256k1::SecretKey::from_slice(config.sender_key.as_bytes())
        .context("sender key must be a 32-byte hex H256")?;

    let sender = {
        let pubkey = secp256k1::PublicKey::from_secret_key(&SECP256K1, &sender_key);
        let hash160 = blake2b_256(&pubkey.serialize())[..20].to_vec();
        Script::new_builder()
            .code_hash(SIGHASH_TYPE_HASH.pack())
            .hash_type(ScriptHashType::Type)
            .args(Bytes::from(hash160).pack())
            .build()
    };

    let signer = SecpCkbRawKeySigner::new_with_secret_keys(vec![sender_key]);
    let sighash_unlocker = SecpSighashUnlocker::from(Box::new(signer) as Box<_>);
    let sighash_script_id = ScriptId::new_type(SIGHASH_TYPE_HASH.clone());
    let mut unlockers = HashMap::new();
    unlockers.insert(
        sighash_script_id,
        Box::new(sighash_unlocker) as Box<dyn ScriptUnlocker>,
    );

    let placeholder_witness = WitnessArgs::new_builder()
        .lock(Some(Bytes::from(vec![0u8; 65])).pack())
        .build();
    let balancer = CapacityBalancer::new_simple(sender, placeholder_witness, config.fee_rate);

    let mut client = CkbRpcClient::new(&config.rpc_url);
    let genesis = client
        .get_block_by_number(0.into())?
        .ok_or_else(|| anyhow!("genesis block was not returned by RPC"))?;
    let cell_dep_resolver = DefaultCellDepResolver::from_genesis(&BlockView::from(genesis))?;
    let header_dep_resolver = DefaultHeaderDepResolver::new(&config.rpc_url);
    let mut cell_collector = DefaultCellCollector::new(&config.rpc_url);
    let tx_dep_provider = DefaultTransactionDependencyProvider::new(&config.rpc_url, 10);

    let output = CellOutput::new_builder()
        .lock(Script::from(&config.receiver))
        .capacity(config.capacity.0)
        .build();

    let builder = CapacityTransferBuilder::new(vec![(output, Bytes::default())]);
    let (tx, still_locked_groups) = builder.build_unlocked(
        &mut cell_collector,
        &cell_dep_resolver,
        &header_dep_resolver,
        &tx_dep_provider,
        &balancer,
        &unlockers,
    )?;

    if !still_locked_groups.is_empty() {
        return Err(anyhow!(
            "transaction still contains locked script groups: {}",
            still_locked_groups.len()
        ));
    }

    let tx_hash = tx.hash();
    println!("transaction_hash={:#x}", tx_hash);

    if config.broadcast {
        let json_tx = JsonTransactionView::from(tx);
        let sent = client.send_transaction(json_tx.inner, Some(OutputsValidator::Passthrough))?;
        println!("broadcast_hash={:#x}", sent);
        Ok(sent)
    } else {
        Ok(tx_hash.into())
    }
}

fn validate_broadcast_network(config: &TransferConfig) -> Result<()> {
    validate_broadcast_rpc_url(&config.rpc_url, config.broadcast)
}

fn validate_broadcast_rpc_url(rpc_url: &str, broadcast: bool) -> Result<()> {
    if !broadcast {
        return Ok(());
    }

    const OFFICIAL_TESTNET_RPC: &str = "https://testnet.ckb.dev";
    if rpc_url.trim_end_matches('/') != OFFICIAL_TESTNET_RPC {
        return Err(anyhow!(
            "broadcast is restricted to the official CKB Testnet RPC: {}",
            OFFICIAL_TESTNET_RPC
        ));
    }

    Ok(())
}

pub fn config_from_env() -> Result<TransferConfig> {
    let rpc_url = env::var("CKB_RPC_URL").unwrap_or_else(|_| "https://testnet.ckb.dev".to_string());
    let sender_key = H256::from_str(
        &env::var("CKB_SENDER_PRIVATE_KEY").context("CKB_SENDER_PRIVATE_KEY is required")?,
    )?;
    let receiver = Address::from_str(
        &env::var("CKB_RECEIVER_ADDRESS").context("CKB_RECEIVER_ADDRESS is required")?,
    )
    .map_err(|error| anyhow!(error))?;
    let capacity = env::var("CKB_TRANSFER_AMOUNT")
        .unwrap_or_else(|_| "61".to_string())
        .parse::<HumanCapacity>()
        .map_err(|error| anyhow!(error))?;
    let fee_rate = env::var("CKB_FEE_RATE")
        .unwrap_or_else(|_| "1000".to_string())
        .parse::<u64>()?;
    let broadcast = env::var("CKB_BROADCAST")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()?;

    Ok(TransferConfig {
        rpc_url,
        sender_key,
        receiver,
        capacity,
        fee_rate,
        broadcast,
    })
}

pub fn run_env_transfer() -> Result<()> {
    let config = config_from_env()?;
    build_and_optionally_send(config)?;
    Ok(())
}

pub fn ensure_send_ready() -> Result<(), Box<dyn Error>> {
    let _ = config_from_env()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::validate_broadcast_rpc_url;

    #[test]
    fn build_only_allows_custom_rpc() {
        assert!(validate_broadcast_rpc_url("http://127.0.0.1:8114", false).is_ok());
    }

    #[test]
    fn broadcast_requires_official_testnet_rpc() {
        assert!(validate_broadcast_rpc_url("https://testnet.ckb.dev", true).is_ok());
        assert!(validate_broadcast_rpc_url("https://mainnet.ckb.dev", true).is_err());
        assert!(validate_broadcast_rpc_url("http://127.0.0.1:8114", true).is_err());
    }
}
