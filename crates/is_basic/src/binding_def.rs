use crate::expr::Expr;
use crate::utils;
use crate::env::Env;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BindingDef {
    pub name: String,
    pub val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let(s, _) = utils::extract_whitespace1(s)?;

        let(s, name) = utils::extract_ident(s)?;
        let(s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s)?;
        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub fn eval(&self, env: &mut Env)-> Result<(), String> {
        env.store_binding(self.name.clone(), self.val.eval(env)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_def() {
        assert_eq!(
            BindingDef::new("let a = 10/2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Box::new(Expr::Number(Number(10))),
                        op: Op::Div,
                        rhs: Box::new(Expr::Number(Number(2))),
                    },
                },
            )),
        );
    }

    #[test]
    fn no_parse_without_space_after_let() {
        assert_eq!(
            BindingDef::new("letaaaa=1+2"),
            Err("expected whitespace".to_string()),
        )
    }
}