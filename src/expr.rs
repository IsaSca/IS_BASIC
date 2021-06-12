 mod binding_usage;
 mod block;

use crate::env::Env;
use crate::utils;
use crate::val::Val;
use binding_usage::BindingUsage;
use block::Block;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
     fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, number) = utils::extract_digits(s)?;
        Ok((s, Self(number.parse().unwrap())))
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Number),
    Operation{
    lhs: Number,
    rhs: Number,
    op: Op},
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s)
        .or_else(|_| Self::new_number(s))
        .or_else(|_| {
            BindingUsage::new(s)
            .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
        })
        .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let(s, lhs) = Number::new(s)?;
        let(s, _) = utils::extract_whitespace(s);

        let(s, op) = Op::new(s)?;
        let(s, _) = utils::extract_whitespace(s);

        let(s, rhs) = Number::new(s)?;

        Ok((s, Self::Operation{lhs, rhs, op}))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::Operation{lhs, rhs, op} => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;
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
                    lhs:Number(1),
                    rhs:Number(2),
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
                    lhs:Number(2),
                    rhs:Number(2),
                    op: Op::Mul,
                }
            ))
        )
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation{
                lhs: Number(10),
                rhs: Number(10),
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
                lhs: Number(10),
                rhs: Number(10),
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
                lhs: Number(10),
                rhs: Number(10),
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
                lhs: Number(10),
                rhs: Number(10),
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

}



