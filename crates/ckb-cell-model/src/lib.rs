#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub capacity: u64,
    pub lock: Script,
    pub type_script: Option<Script>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    pub code_hash: [u8; 32],
    pub args: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellOutPoint {
    pub tx_hash: String,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveCell {
    pub out_point: CellOutPoint,
    pub capacity: u64,
    pub lock: Script,
    pub type_script: Option<Script>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellTransaction {
    pub inputs: Vec<Cell>,
    pub outputs: Vec<Cell>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    CapacityNotConserved { inputs: u64, outputs: u64 },
    CapacityOverflow,
}

impl CellTransaction {
    pub fn validate_capacity(&self) -> Result<(), ValidationError> {
        let inputs = self.inputs.iter().try_fold(0u64, |sum, cell| {
            sum.checked_add(cell.capacity)
                .ok_or(ValidationError::CapacityOverflow)
        })?;
        let outputs = self.outputs.iter().try_fold(0u64, |sum, cell| {
            sum.checked_add(cell.capacity)
                .ok_or(ValidationError::CapacityOverflow)
        })?;

        if inputs == outputs {
            Ok(())
        } else {
            Err(ValidationError::CapacityNotConserved { inputs, outputs })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(capacity: u64) -> Cell {
        Cell {
            capacity,
            lock: Script {
                code_hash: [0; 32],
                args: vec![],
            },
            type_script: None,
            data: vec![],
        }
    }

    #[test]
    fn accepts_conserved_capacity() {
        let tx = CellTransaction {
            inputs: vec![cell(100), cell(50)],
            outputs: vec![cell(120), cell(30)],
        };
        assert!(tx.validate_capacity().is_ok());
    }

    #[test]
    fn rejects_capacity_loss() {
        let tx = CellTransaction {
            inputs: vec![cell(100)],
            outputs: vec![cell(99)],
        };
        assert_eq!(
            tx.validate_capacity(),
            Err(ValidationError::CapacityNotConserved {
                inputs: 100,
                outputs: 99
            })
        );
    }

    #[test]
    fn rejects_capacity_sum_overflow() {
        let tx = CellTransaction {
            inputs: vec![cell(u64::MAX), cell(1)],
            outputs: vec![],
        };
        assert_eq!(
            tx.validate_capacity(),
            Err(ValidationError::CapacityOverflow)
        );
    }
}
