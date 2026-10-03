use ckb_sdk::rpc::CkbRpcClient;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CkbNodeInfo {
    pub rpc_url: String,
    pub tip_block_number: u64,
}

#[derive(Debug)]
pub enum RpcAdapterError {
    Rpc(String),
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
