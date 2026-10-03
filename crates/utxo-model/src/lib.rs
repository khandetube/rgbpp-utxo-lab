use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub inputs: Vec<Utxo>,
    pub total: u64,
    pub change: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionError {
    InsufficientFunds { available: u64, required: u64 },
}

pub fn select_largest_first(mut utxos: Vec<Utxo>, target: u64) -> Result<Selection, SelectionError> {
    utxos.sort_by(|a, b| b.value.cmp(&a.value).then_with(|| a.outpoint.txid.cmp(&b.outpoint.txid)));
    let mut selected = Vec::new();
    let mut total = 0u64;
    for u in utxos {
        selected.push(u);
        total = match total.checked_add(selected.last().unwrap().value) {
            Some(value) => value,
            None => return Err(SelectionError::InsufficientFunds { available: u64::MAX, required: target }),
        };
        if total >= target {
            return Ok(Selection { inputs: selected, total, change: total - target });
        }
    }
    Err(SelectionError::InsufficientFunds { available: total, required: target })
}

pub fn index_by_outpoint(utxos: &[Utxo]) -> BTreeMap<(String, u32), u64> {
    utxos.iter().map(|u| ((u.outpoint.txid.clone(), u.outpoint.vout), u.value)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u(txid: &str, vout: u32, value: u64) -> Utxo { Utxo { outpoint: OutPoint { txid: txid.into(), vout }, value } }

    #[test]
    fn selects_deterministically_and_returns_change() {
        let s = select_largest_first(vec![u("a",0,40), u("b",0,100), u("c",0,60)], 130).unwrap();
        assert_eq!(s.total, 160);
        assert_eq!(s.change, 30);
        assert_eq!(s.inputs.len(), 2);
    }

    #[test]
    fn rejects_insufficient_funds() {
        let e = select_largest_first(vec![u("a",0,10)], 11).unwrap_err();
        assert_eq!(e, SelectionError::InsufficientFunds { available: 10, required: 11 });
    }
}


/// Parse and normalize a Bitcoin transaction outpoint using rust-bitcoin.
pub fn parse_bitcoin_outpoint(value: &str) -> Result<bitcoin::OutPoint, bitcoin::transaction::ParseOutPointError> {
    value.parse()
}

#[cfg(test)]
mod bitcoin_tests {
    use super::*;

    #[test]
    fn parses_bitcoin_outpoint() {
        let txid = "0000000000000000000000000000000000000000000000000000000000000001";
        let outpoint = parse_bitcoin_outpoint(&format!("{txid}:0")).unwrap();
        assert_eq!(outpoint.vout, 0);
    }
}
