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

        let(s, param) = Expr::new(s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params: vec![param],
            },
        ))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Number;

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
}