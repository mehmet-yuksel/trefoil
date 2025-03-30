use crate::ast::Ast;
use std::iter::Peekable;
use std::slice::Iter;

pub fn tokenize(code: &str) -> Vec<String> {
    code.replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse(tokens: &[String]) -> Result<Ast, String> {
    let mut iter = tokens.iter().peekable();
    let mut top_level_nodes = Vec::new();

    while iter.peek().is_some() {
        if iter.peek() == Some(&&")".to_string()) {
            return Err("Unexpected closing parenthesis ')' at top level.".to_string());
        }
        top_level_nodes.push(parse_expression(&mut iter)?);
    }

    Ok(Ast::List(top_level_nodes))
}

fn parse_expression(iter: &mut Peekable<Iter<'_, String>>) -> Result<Ast, String> {
    let token = match iter.next() {
        Some(token) => token,
        None => return Err("Unexpected end of input while expecting an expression.".to_string()),
    };

    match token.as_str() {
        "(" => parse_list_items(iter),
        ")" => Err(
            "Unexpected closing parenthesis ')' when expecting an expression start or atom."
                .to_string(),
        ),
        _ => Ok(Ast::Atom(token.clone())),
    }
}

fn parse_list_items(iter: &mut Peekable<Iter<'_, String>>) -> Result<Ast, String> {
    let mut list_nodes = Vec::new();
    loop {
        match iter.peek() {
            Some(&token) => {
                if token == ")" {
                    iter.next();
                    return Ok(Ast::List(list_nodes));
                } else {
                    list_nodes.push(parse_expression(iter)?);
                }
            }
            None => return Err("Missing closing parenthesis ')' for list.".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ast::{Atom, List};

    #[test]
    fn test_tokenize_simple() {
        let code = "(define x 10)";
        let tokens = tokenize(code);
        assert_eq!(tokens, vec!["(", "define", "x", "10", ")"]);
    }

    #[test]
    fn test_tokenize_multiple() {
        let code = "(define x 10)(print x)";
        let tokens = tokenize(code);
        assert_eq!(
            tokens,
            vec!["(", "define", "x", "10", ")", "(", "print", "x", ")"]
        );
    }

    #[test]
    fn test_parse_single_expression() {
        let tokens = tokenize("(define x 10)");
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            List(vec![List(vec![
                Atom("define".to_string()),
                Atom("x".to_string()),
                Atom("10".to_string())
            ])])
        );
    }

    #[test]
    fn test_parse_multiple_expressions() {
        let tokens = tokenize("(define x 10) (print x)");
        let ast = parse(&tokens).unwrap();
        // Expect a top-level list containing the two parsed lists
        assert_eq!(
            ast,
            List(vec![
                List(vec![
                    Atom("define".to_string()),
                    Atom("x".to_string()),
                    Atom("10".to_string())
                ]),
                List(vec![Atom("print".to_string()), Atom("x".to_string())])
            ])
        );
    }

    #[test]
    fn test_parse_nested_list() {
        let tokens = tokenize("(list 1 (list 2 3))");
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            List(vec![
                // Top-level list wrapper
                List(vec![
                    Atom("list".to_string()),
                    Atom("1".to_string()),
                    List(vec![
                        Atom("list".to_string()),
                        Atom("2".to_string()),
                        Atom("3".to_string())
                    ])
                ])
            ])
        );
    }

    #[test]
    fn test_parse_empty_input() {
        let tokens = tokenize("");
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, List(vec![]));
    }

    #[test]
    fn test_parse_empty_list() {
        let tokens = tokenize("()");
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, List(vec![List(vec![])]));
    }

    #[test]
    fn test_parse_multiple_empty_lists() {
        let tokens = tokenize("() ()");
        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, List(vec![List(vec![]), List(vec![])]));
    }

    #[test]
    fn test_parse_error_unmatched_closing_paren() {
        let tokens = tokenize("(define x 10))");
        let result = parse(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Unexpected closing parenthesis ')' at top level."
        );

        let tokens_leading = tokenize(") (define x 10)");
        let result_leading = parse(&tokens_leading);
        assert!(result_leading.is_err());
        assert_eq!(
            result_leading.err().unwrap(),
            "Unexpected closing parenthesis ')' at top level."
        );
    }

    #[test]
    fn test_parse_error_unmatched_opening_paren() {
        let tokens = tokenize("(define x 10");
        let result = parse(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Missing closing parenthesis ')' for list."
        );

        let tokens_nested = tokenize("(list 1 (list 2 3");
        let result_nested = parse(&tokens_nested);
        assert!(result_nested.is_err());
        assert_eq!(
            result_nested.err().unwrap(),
            "Missing closing parenthesis ')' for list."
        );
    }

    #[test]
    fn test_parse_error_unexpected_end() {
        let tokens = tokenize("(define x");
        let result = parse(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Missing closing parenthesis ')' for list."
        );
    }
}
