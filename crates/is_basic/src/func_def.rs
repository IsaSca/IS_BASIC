use crate::stmt::Stmt;
use crate::utils;

#[derive(Debug, PartialEq)]
pub(crate) struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Stmt>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn",s)?;
        let(s, _) = utils::extract_whitespace1(s)?;

        let(s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace(s);

        let(s, params) = utils::sequence(
            |s| utils::extract_ident(s).map(|(s, ident)| (s, ident.to_string())),s,
        )?;

        let s = utils::tag("=>", s)?;
        let(s, _) = utils::extract_whitespace(s);

        let(s, body) = Stmt::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                params,
                body: Box::new(body)
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Block, Expr, BindingUsage, Op};

    #[test]
    fn parse_func_def_no_params_no_body() {
        assert_eq!(
            FuncDef::new("fn nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Stmt::Expr(Expr::Block(Block {stmts: Vec::new()}))),
                },
            )),
        );
    }

    #[test]
    fn parse_func_one_param_no_body() {
        assert_eq!(
            FuncDef::new("fn id x => {}"),
            Ok((
                "",
                FuncDef {
                    name: "id".to_string(),
                    params: vec!["x".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Block(Block{stmts: Vec::new()}))),
                }
            ))
        )
    }

    #[test]
    fn parse_func_multiparam_some_body() {
        assert_eq!(
            FuncDef::new("fn add x y => x+y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "x".to_string(),
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "y".to_string(),
                        })),
                        op: Op::Add,
                    })),
                },
            )),
        );
    }

}