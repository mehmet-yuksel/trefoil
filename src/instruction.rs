use crate::ast::Ast;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Instruction {
    Insert {
        path: Vec<usize>,
        index: usize,
        node: Ast,
    },
    Delete {
        path: Vec<usize>,
        index: usize,
    },
    Update {
        path: Vec<usize>,
        new_value: String,
    },
    Replace {
        path: Vec<usize>,
        node: Ast,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Instruction::Insert { path, index, node } => write!(
                f,
                "Insert at path {:?} index {} node {}",
                path,
                index,
                node
            ),
            Instruction::Delete { path, index } => {
                write!(f, "Delete at path {:?} index {}", path, index)
            }
            Instruction::Update { path, new_value } => {
                write!(f, "Update at path {:?} with value {}", path, new_value)
            }
            Instruction::Replace { path, node } => {
                write!(f, "Replace at path {:?} with {}", path, node)
            }
        }
    }
}
