use crate::stmt::Stmt;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let(s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block {stmts: Vec::new()}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts:Vec::new()})))
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(Block::new("{    }"), Ok(("", Block { stmts: Vec::new()})));
    }

    #[test]
    fn parse_block_with_value() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "", Block {
                    stmts:vec![Stmt::Expr(Expr::Number(Number(5)))],
                },
            )),
        );
    }

}