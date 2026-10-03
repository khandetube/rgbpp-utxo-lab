use serde_json::{json, Value};

use crate::RpcAdapterError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedCell {
    pub tx_hash: String,
    pub output_index: u32,
    pub capacity_shannons: u64,
    pub output_data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellPage {
    pub cells: Vec<IndexedCell>,
    pub next_cursor: Option<String>,
}

/// Query live cells controlled by an exact lock script.
/// All returned values come from the connected CKB indexer.
pub fn collect_cells_by_lock(
    rpc_url: &str,
    lock_script: Value,
    limit: u32,
    cursor: Option<&str>,
    with_data: bool,
) -> Result<CellPage, RpcAdapterError> {
    if !lock_script.is_object() {
        return Err(RpcAdapterError::InvalidRequest(
            "lock_script must be a JSON object".to_owned(),
        ));
    }
    if !(1..=1000).contains(&limit) {
        return Err(RpcAdapterError::InvalidRequest(
            "limit must be between 1 and 1000".to_owned(),
        ));
    }

    let client = ckb_sdk::rpc::CkbRpcClient::new(rpc_url);
    let search_key = json!({
        "script": lock_script,
        "script_type": "lock",
        "script_search_mode": "exact",
        "with_data": with_data,
        "group_by_transaction": false
    });

    let params = json!([
        search_key,
        "asc",
        format!("0x{limit:x}"),
        cursor.map(Value::from).unwrap_or(Value::Null)
    ]);

    let response: Value = client
        .post("get_cells", params)
        .map_err(|error| RpcAdapterError::Rpc(error.to_string()))?;

    parse_cell_page(response)
}

fn parse_cell_page(response: Value) -> Result<CellPage, RpcAdapterError> {
    let objects = response
        .get("objects")
        .and_then(Value::as_array)
        .ok_or_else(|| RpcAdapterError::InvalidResponse("missing objects".to_owned()))?;

    let mut cells = Vec::with_capacity(objects.len());

    for object in objects {
        let out_point = object
            .get("out_point")
            .ok_or_else(|| RpcAdapterError::InvalidResponse("missing out_point".to_owned()))?;

        let tx_hash = out_point
            .get("tx_hash")
            .and_then(Value::as_str)
            .ok_or_else(|| RpcAdapterError::InvalidResponse("missing tx_hash".to_owned()))?
            .to_owned();

        let output_index = parse_hex_u32(
            out_point
                .get("index")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    RpcAdapterError::InvalidResponse("missing output index".to_owned())
                })?,
        )?;

        let capacity_shannons = parse_hex_u64(
            object
                .get("output")
                .and_then(|output| output.get("capacity"))
                .and_then(Value::as_str)
                .ok_or_else(|| RpcAdapterError::InvalidResponse("missing capacity".to_owned()))?,
        )?;

        let output_data = object
            .get("output_data")
            .and_then(Value::as_str)
            .map(str::to_owned);

        cells.push(IndexedCell {
            tx_hash,
            output_index,
            capacity_shannons,
            output_data,
        });
    }

    let next_cursor = response
        .get("last_cursor")
        .and_then(Value::as_str)
        .filter(|cursor| !cursor.is_empty())
        .map(str::to_owned);

    Ok(CellPage {
        cells,
        next_cursor,
    })
}

fn parse_hex_u32(value: &str) -> Result<u32, RpcAdapterError> {
    u32::from_str_radix(value.trim_start_matches("0x"), 16)
        .map_err(|_| RpcAdapterError::InvalidResponse(format!("invalid u32: {value}")))
}

fn parse_hex_u64(value: &str) -> Result<u64, RpcAdapterError> {
    u64::from_str_radix(value.trim_start_matches("0x"), 16)
        .map_err(|_| RpcAdapterError::InvalidResponse(format!("invalid u64: {value}")))
}

#[cfg(test)]
mod tests {
    use super::parse_cell_page;
    use serde_json::json;

    #[test]
    fn parses_indexed_cell_page() {
        let page = parse_cell_page(json!({
            "objects": [{
                "output": { "capacity": "0x174876e800" },
                "output_data": "0x",
                "out_point": {
                    "index": "0x2",
                    "tx_hash": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                }
            }],
            "last_cursor": "0x01"
        }))
        .unwrap();

        assert_eq!(page.cells.len(), 1);
        assert_eq!(page.cells[0].output_index, 2);
        assert_eq!(page.cells[0].capacity_shannons, 100_000_000_000);
        assert_eq!(page.next_cursor.as_deref(), Some("0x01"));
    }

    #[test]
    fn rejects_malformed_page() {
        assert!(parse_cell_page(json!({ "objects": [{}] })).is_err());
    }
}
