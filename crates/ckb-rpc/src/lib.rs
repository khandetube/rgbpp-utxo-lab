use ckb_sdk::rpc::CkbRpcClient;
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CkbNodeInfo {
    pub rpc_url: String,
    pub tip_block_number: u64,
}

#[derive(Debug)]
pub enum RpcAdapterError {
    Rpc(String),
    InvalidTransactionHash(String),
    InvalidRequest(String),
    InvalidResponse(String),
}

pub fn fetch_tip_block_number(rpc_url: &str) -> Result<u64, RpcAdapterError> {
    let client = CkbRpcClient::new(rpc_url);
    client
        .get_tip_block_number()
        .map(|number| number.value())
        .map_err(|error| RpcAdapterError::Rpc(error.to_string()))
}

pub fn inspect_node(rpc_url: &str) -> Result<CkbNodeInfo, RpcAdapterError> {
    let tip_block_number = fetch_tip_block_number(rpc_url)?;
    Ok(CkbNodeInfo {
        rpc_url: rpc_url.to_owned(),
        tip_block_number,
    })
}

/// Query a CKB cell directly by transaction hash and output index.
///
/// This deliberately returns the node's JSON response rather than inventing a
/// local representation of every CKB RPC field. The official SDK remains the
/// network boundary, while the portfolio's domain models stay protocol-focused.
pub fn get_live_cell(
    rpc_url: &str,
    tx_hash: &str,
    output_index: u32,
    with_data: bool,
) -> Result<Value, RpcAdapterError> {
    if !is_valid_h256(tx_hash) {
        return Err(RpcAdapterError::InvalidTransactionHash(
            tx_hash.to_owned(),
        ));
    }

    let client = CkbRpcClient::new(rpc_url);
    let params = json!([
        {
            "tx_hash": normalize_hex(tx_hash),
            "index": format!("0x{output_index:x}")
        },
        with_data
    ]);

    client
        .post("get_live_cell", params)
        .map_err(|error| RpcAdapterError::Rpc(error.to_string()))
}

fn normalize_hex(value: &str) -> String {
    value.strip_prefix("0x").unwrap_or(value).to_owned()
}

fn is_valid_h256(value: &str) -> bool {
    let raw = value.strip_prefix("0x").unwrap_or(value);
    raw.len() == 64 && raw.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::{is_valid_h256, normalize_hex};

    #[test]
    fn validates_32_byte_hashes() {
        assert!(is_valid_h256(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        ));
        assert!(!is_valid_h256("0x1234"));
    }

    #[test]
    fn normalizes_prefixed_hex() {
        assert_eq!(normalize_hex("0xabcd"), "abcd");
        assert_eq!(normalize_hex("abcd"), "abcd");
    }
}
