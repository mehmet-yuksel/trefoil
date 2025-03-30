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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply_instruction;
    use proptest::prelude::*;
    use proptest::strategy::{BoxedStrategy, Strategy};

    fn atom_strategy() -> BoxedStrategy<Ast> {
        prop::string::string_regex("[a-zA-Z0-9_]+")
            .unwrap()
            .prop_map(|s| Ast::Atom(s))
            .boxed()
    }

    fn list_strategy(depth: u32) -> BoxedStrategy<Ast> {
        if depth == 0 {
            atom_strategy()
        } else {
            prop::collection::vec(ast_strategy(depth - 1), 0..5)
                .prop_map(|v| Ast::List(v))
                .boxed()
        }
    }

    fn ast_strategy(depth: u32) -> BoxedStrategy<Ast> {
        if depth == 0 {
            atom_strategy()
        } else {
            proptest::prop_oneof![
                atom_strategy(),
                list_strategy(depth - 1)
            ].boxed()
        }
    }

    proptest! {
        #[test]
        fn diff_apply_roundtrip(old in ast_strategy(3), new in ast_strategy(3)) {
            let mut path = vec![];
            let instructions = diff_ast(&old, &new, &mut path);
            
            // Apply all instructions to the old AST
            let mut result = old.clone();
            for instruction in instructions {
                result = apply_instruction(result, instruction);
            }

            // The result should equal the new AST
            assert_eq!(result, new);
        }

        #[test]
        fn diff_minimal(old in ast_strategy(3), new in ast_strategy(3)) {
            let mut path = vec![];
            let instructions = diff_ast(&old, &new, &mut path);

            // If old and new are equal, there should be no instructions
            if old == new {
                assert!(instructions.is_empty());
            }
        }
    }
}
