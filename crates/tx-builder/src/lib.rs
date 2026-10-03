use utxo_model::{select_largest_first, Selection, SelectionError, Utxo};

pub fn build_transfer(utxos: Vec<Utxo>, amount: u64, fee: u64) -> Result<Selection, SelectionError> {
    let target = amount.checked_add(fee).expect("amount + fee overflow");
    select_largest_first(utxos, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utxo_model::OutPoint;

    #[test]
    fn fee_is_accounted_for_before_selection() {
        let u = Utxo { outpoint: OutPoint { txid: "demo".into(), vout: 0 }, value: 105 };
        let s = build_transfer(vec![u], 100, 5).unwrap();
        assert_eq!(s.change, 0);
    }
}
