mod binding_usage;
mod block;

pub(crate) use binding_usage::BindingUsage;
pub(crate) use block::Block;

use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub i32);

impl Number {
     fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, number) = utils::extract_digits(s)?;
        Ok((s, Self(number.parse().unwrap())))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
     fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+",s)
            .map(|s|(s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s|(s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s|(s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s|(s, Self::Div)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expr {
    Number(Number),
    Operation{
        lhs: Box<Self>,
        rhs: Box<Self>,
        op: Op
    },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {

    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s).or_else(|_| Self:: new_non_operation(s))
    }

    pub fn new_non_operation(s:&str) -> Result<(&str, Self), String> {
        Self::new_number(s)
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s,Self::BindingUsage(binding_usage)))
            })
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let(s, lhs) = Self::new(s)?;
        let(s, _) = utils::extract_whitespace(s);

        let(s, op) = Op::new(s)?;
        let(s, _) = utils::extract_whitespace(s);

        let(s, rhs) = Self::new_non_operation(s)?;

        Ok((s,
            Self::Operation{
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op
            },
        ))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::Operation{lhs, rhs, op} => {
                let lhs = lhs.eval(env)?;
                let rhs = rhs.eval(env)?;

                let (lhs, rhs) = match (lhs, rhs) {
                    (Val::Number(lhs), Val::Number(rhs)) => (lhs, rhs),
                    _ => return Err("cannot evaluate operation where left-hand and right-hand are not numbers".to_string()),
                };

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };
                Ok(Val::Number(result))
            }
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stmt::Stmt;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
    }

    #[test]
    fn parse_multi() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));

    }

    #[test]
    fn parse_division() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Add,
                }
            ))

        );
    }

    #[test]
    fn expr_extract_w_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(2))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Mul,
                }
            ))
        )
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation{
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Number(Number(10))),
                op: Op::Add,
            }
            .eval(&Env::default()),
            Ok(Val::Number(20)),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation{
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Number(Number(10))),
                op: Op::Sub,
            }
            .eval(&Env::default()),
            Ok(Val::Number(0)),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation{
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Number(Number(10))),
                op: Op::Mul,
            }
            .eval(&Env::default()),
            Ok(Val::Number(100)),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation{
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Number(Number(10))),
                op: Op::Div,
            }
            .eval(&Env::default()),
            Ok(Val::Number(1)),
        );
    }

    #[test]
    fn eval_block() {
        assert_eq!(
            Expr::Block(Block {
                stmts: vec![Stmt::Expr(Expr::Number(Number(10)))],
            })
            .eval(&Env::default()),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn parse_as_expr() {
        assert_eq!(Expr::new("458"), Ok(("", Expr::Number(Number(458)))));
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("bar"), 
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_string(),
                }),
            )),
        );
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expr::new("{ 200 }"),
            Ok((
                "",
                Expr::Block(Block {
                    stmts:vec![Stmt::Expr(Expr::Number(Number(200)))],
                }),
            )),
        );
    }
    
    #[test]
    fn eval_binding_use() {
        let mut env = Env::default();
        env.store_binding("ten".to_string(), Val::Number(10));

        assert_eq!(
            Expr::BindingUsage(BindingUsage {
                name:"ten".to_string(),
            })
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_non_number_op() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Block(Block {stmts: Vec::new()})),
                op: Op::Add,
            }
                .eval(&Env::default()),
            Err("Cannot evaluate operation where left-hand and right-hand are not numbers".to_string()),
        );
    }

}



