use crate::ast::Ast;
use crate::instruction::Instruction;

pub fn diff_ast(old: &Ast, new: &Ast, path: &mut Vec<usize>) -> Vec<Instruction> {
    match (old, new) {
        (Ast::Atom(a), Ast::Atom(b)) => {
            if a != b {
                vec![Instruction::Update {
                    path: path.clone(),
                    new_value: b.clone(),
                }]
            } else {
                vec![]
            }
        }
        (Ast::List(old_children), Ast::List(new_children)) => {
            if old_children.len() == new_children.len() {
                let mut instructions = Vec::new();
                for (i, (old_child, new_child)) in
                    old_children.iter().zip(new_children.iter()).enumerate()
                {
                    path.push(i);
                    instructions.extend(diff_ast(old_child, new_child, path));
                    path.pop();
                }
                instructions
            } else {
                vec![Instruction::Replace {
                    path: path.clone(),
                    node: new.clone(),
                }]
            }
        }
        _ => {
            vec![Instruction::Replace {
                path: path.clone(),
                node: new.clone(),
            }]
        }
    }
}
