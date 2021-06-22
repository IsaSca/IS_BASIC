use super::Expr;
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_ident(s)?;
        let(s, _) = utils::extract_whitespace1(s)?;

        let(s, params) = utils::sequence1(Expr::new, |s| utils::take_while(|c| c == ' ', s), s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params,
            },
        ))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Number;
    use crate::expr::{BindingUsage, Op};
    use crate::stmt::Stmt;
    use crate::func_def::FuncDef;

    #[test]
    fn parse_func_call_no_param() {
        assert_eq!(
            FuncCall::new("hello_user"),
            Ok((
                "",
                FuncCall {
                    callee: "hello_user".to_string(),
                    params: Vec::new(),
                },
            )),
        );
    }

    #[test]
    fn parse_func_call_one_param() {
        assert_eq!(
            FuncCall::new("factorial 10"),
            Ok((
                "",
                FuncCall {
                    callee: "factorial".to_string(),
                    params: vec![Expr::Number(Number(10))],
                },
            )),
        );
    }
    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::new("fn add x y => x + y"),
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