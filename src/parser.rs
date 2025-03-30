use crate::ast::Ast;

pub fn tokenize(code: &str) -> Vec<String> {
    code.replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse(tokens: &[String]) -> Result<Ast, String> {
    let mut iter = tokens.iter();
    parse_expression(&mut iter)
}

fn parse_expression(iter: &mut std::slice::Iter<'_, String>) -> Result<Ast, String> {
    let token = iter.next().ok_or("Unexpected end of input")?;
    if token == "(" {
        let mut list = Vec::new();
        while let Some(next_token) = iter.clone().next() {
            if next_token == ")" {
                iter.next(); // Consume ")"
                return Ok(Ast::List(list));
            } else {
                list.push(parse_expression(iter)?);
            }
        }
        Err("Missing closing parenthesis".to_string())
    } else if token == ")" {
        Err("Unexpected closing parenthesis".to_string())
    } else {
        Ok(Ast::Atom(token.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let code = "(define x 10)";
        let tokens = tokenize(code);
        assert_eq!(tokens, vec!["(", "define", "x", "10", ")"]);
    }

    #[test]
    fn test_parse() {
        let tokens = tokenize("(define x 10)");
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Ast::List(vec![
                Ast::Atom("define".to_string()),
                Ast::Atom("x".to_string()),
                Ast::Atom("10".to_string())
            ])
        );
    }
}
