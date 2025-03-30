use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    Atom(String),
    List(Vec<Ast>),
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Atom(ref s) => write!(f, "{}", s),
            Ast::List(ref v) => {
                write!(f, "(")?;
                for (i, item) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
        }
    }
}
