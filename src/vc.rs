use crate::instruction::Instruction;
use serde::{Serialize, Deserialize};
use crate::ast::Ast;
use crate::apply::apply_instruction;
use std::error::Error;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Commit {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub instructions: Vec<Instruction>,
    pub timestamp: u64,
}

pub fn save_commit(commit: &Commit, dir: &Path) -> Result<(), Box<dyn Error>> {
    let encoded = serde_json::to_string(commit)?;
    let path = dir.join(format!("{}.json", commit.id));
    std::fs::write(path, encoded)?;
    Ok(())
}

pub fn load_commit(id: u64, dir: &Path) -> Result<Commit, Box<dyn Error>> {
    let path = dir.join(format!("{}.json", id));
    let data = std::fs::read_to_string(path)?;
    let commit: Commit = serde_json::from_str(&data)?;
    Ok(commit)
}

pub fn get_current_commit_id(dir: &Path) -> Result<u64, Box<dyn Error>> {
    let head_path = dir.join("HEAD");
    let id_str = std::fs::read_to_string(head_path)?;
    let id = id_str.trim().parse::<u64>()?;
    Ok(id)
}

pub fn set_current_commit_id(id: u64, dir: &Path) -> Result<(), Box<dyn Error>> {
    let head_path = dir.join("HEAD");
    std::fs::write(head_path, id.to_string())?;
    Ok(())
}

pub fn get_commit_chain(current_id: u64, dir: &Path) -> Result<Vec<Commit>, Box<dyn Error>> {
    let mut chain = Vec::new();
    let mut id = Some(current_id);
    while let Some(current_id) = id {
        let commit = load_commit(current_id, dir)?;
        id = commit.parent_id;
        chain.push(commit);
    }
    Ok(chain)
}

pub fn reconstruct_ast(up_to_id: u64, dir: &Path) -> Result<Ast, Box<dyn Error>> {
    let mut ast = Ast::List(vec![]); // Initial empty list
    let chain = get_commit_chain(up_to_id, dir)?;
    for commit in chain.iter().rev() { // From root to up_to_id
        for instruction in &commit.instructions {
            ast = apply_instruction(ast, instruction.clone());
        }
    }
    Ok(ast)
}
