use crate::ast::Ast;
use crate::instruction::Instruction;

pub fn apply_instruction(ast: Ast, instruction: Instruction) -> Ast {
    match instruction {
        Instruction::Insert { path, index, node } => apply_insert(ast, &path, index, node),
        Instruction::Delete { path, index } => apply_delete(ast, &path, index),
        Instruction::Update { path, new_value } => apply_update(ast, &path, new_value),
        Instruction::Replace { path, node } => apply_replace(ast, &path, node),
    }
}

fn apply_insert(ast: Ast, path: &[usize], index: usize, node: Ast) -> Ast {
    if path.is_empty() {
        if let Ast::List(mut list) = ast {
            list.insert(index, node);
            Ast::List(list)
        } else {
            panic!("Expected list at path");
        }
    } else {
        let next_index = path[0];
        if let Ast::List(list) = ast {
            let mut new_list = list.clone();
            let child = new_list[next_index].clone();
            let new_child = apply_insert(child, &path[1..], index, node);
            new_list[next_index] = new_child;
            Ast::List(new_list)
        } else {
            panic!("Expected list at path");
        }
    }
}

fn apply_delete(ast: Ast, path: &[usize], index: usize) -> Ast {
    if path.is_empty() {
        if let Ast::List(mut list) = ast {
            if index < list.len() {
                list.remove(index);
                Ast::List(list)
            } else {
                panic!("Index out of bounds");
            }
        } else {
            panic!("Expected list at path");
        }
    } else {
        let next_index = path[0];
        if let Ast::List(list) = ast {
            let mut new_list = list.clone();
            let child = new_list[next_index].clone();
            let new_child = apply_delete(child, &path[1..], index);
            new_list[next_index] = new_child;
            Ast::List(new_list)
        } else {
            panic!("Expected list at path");
        }
    }
}

fn apply_update(ast: Ast, path: &[usize], new_value: String) -> Ast {
    if path.is_empty() {
        // Handle root-level atom update
        match ast {
            Ast::Atom(_) => Ast::Atom(new_value),
            _ => panic!("Expected atom when updating with empty path"),
        }
    } else if path.len() == 1 {
        // Direct child of the root
        let index = path[0];
        if let Ast::List(mut list) = ast {
            if index < list.len() {
                if let Ast::Atom(_) = list[index] {
                    list[index] = Ast::Atom(new_value);
                    Ast::List(list)
                } else {
                    panic!("Expected atom at path");
                }
            } else {
                panic!("Index out of bounds");
            }
        } else {
            panic!("Expected list at root");
        }
    } else {
        // Handle nested paths
        let first_index = path[0];
        let rest_of_path = &path[1..];

        if let Ast::List(mut list) = ast {
            if first_index < list.len() {
                list[first_index] =
                    apply_update(list[first_index].clone(), rest_of_path, new_value);
                Ast::List(list)
            } else {
                panic!("Index out of bounds at path segment");
            }
        } else {
            panic!("Expected list at path segment");
        }
    }
}

fn apply_replace(ast: Ast, path: &[usize], node: Ast) -> Ast {
    if path.is_empty() {
        node
    } else {
        let next_index = path[0];
        if let Ast::List(list) = ast {
            let mut new_list = list.clone();
            new_list[next_index] = apply_replace(new_list[next_index].clone(), &path[1..], node);
            Ast::List(new_list)
        } else {
            panic!("Expected list at path");
        }
    }
}
