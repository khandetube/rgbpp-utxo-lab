use ckb_cell_model::{CellOutPoint, LiveCell};
use utxo_model::{select_largest_first, Selection, SelectionError, Utxo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellSelection {
    pub inputs: Vec<LiveCell>,
    pub total_capacity: u64,
    pub target_capacity: u64,
    pub change_capacity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellSelectionError {
    InvalidTarget,
    InsufficientCapacity { available: u64, required: u64 },
    CapacityOverflow,
}

pub fn select_cells_for_transfer(
    cells: Vec<LiveCell>,
    amount: u64,
    fee: u64,
) -> Result<CellSelection, CellSelectionError> {
    let target_capacity = amount
        .checked_add(fee)
        .ok_or(CellSelectionError::InvalidTarget)?;

    let mut ordered = cells;
    ordered.sort_by(|a, b| {
        b.capacity
            .cmp(&a.capacity)
            .then_with(|| a.out_point.tx_hash.cmp(&b.out_point.tx_hash))
            .then_with(|| a.out_point.index.cmp(&b.out_point.index))
    });

    let mut inputs = Vec::new();
    let mut total_capacity = 0u64;

    for cell in ordered {
        total_capacity = total_capacity
            .checked_add(cell.capacity)
            .ok_or(CellSelectionError::CapacityOverflow)?;
        inputs.push(cell);

        if total_capacity >= target_capacity {
            let change_capacity = total_capacity - target_capacity;
            return Ok(CellSelection {
                inputs,
                total_capacity,
                target_capacity,
                change_capacity,
            });
        }
    }

    Err(CellSelectionError::InsufficientCapacity {
        available: total_capacity,
        required: target_capacity,
    })
}

pub fn build_transfer(
    utxos: Vec<Utxo>,
    amount: u64,
    fee: u64,
) -> Result<Selection, SelectionError> {
    let target = amount.checked_add(fee).ok_or(SelectionError::InsufficientFunds)?;
    select_largest_first(utxos, target)
}

pub fn cell_out_point(tx_hash: impl Into<String>, index: u32) -> CellOutPoint {
    CellOutPoint { tx_hash: tx_hash.into(), index }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ckb_cell_model::Script;
    use utxo_model::OutPoint;

    fn cell(tx_hash: &str, index: u32, capacity: u64) -> LiveCell {
        LiveCell {
            out_point: CellOutPoint { tx_hash: tx_hash.into(), index },
            capacity,
            lock: Script { code_hash: [0; 32], args: vec![] },
            type_script: None,
            data: vec![],
        }
    }

    #[test]
    fn selects_deterministically_and_returns_change() {
        let cells = vec![
            cell("0xbb", 0, 60),
            cell("0xaa", 0, 60),
            cell("0xcc", 0, 10),
        ];

        let selected = select_cells_for_transfer(cells, 100, 5).unwrap();

        assert_eq!(selected.total_capacity, 120);
        assert_eq!(selected.target_capacity, 105);
        assert_eq!(selected.change_capacity, 15);
        assert_eq!(selected.inputs[0].out_point.tx_hash, "0xaa");
        assert_eq!(selected.inputs[1].out_point.tx_hash, "0xbb");
    }

    #[test]
    fn rejects_insufficient_capacity() {
        let selected = select_cells_for_transfer(vec![cell("0xaa", 0, 50)], 100, 1);
        assert_eq!(
            selected,
            Err(CellSelectionError::InsufficientCapacity { available: 50, required: 101 })
        );
    }

    #[test]
    fn rejects_target_overflow() {
        let selected = select_cells_for_transfer(vec![], u64::MAX, 1);
        assert_eq!(selected, Err(CellSelectionError::InvalidTarget));
    }

    #[test]
    fn fee_is_accounted_for_before_selection() {
        let u = Utxo { outpoint: OutPoint { txid: "demo".into(), vout: 0 }, value: 105 };
        let s = build_transfer(vec![u], 100, 5).unwrap();
        assert_eq!(s.change, 0);
    }
}
