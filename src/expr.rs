use crate::utils;
use crate::val::Val;


#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);
        (s, Self(number.parse().unwrap()))
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
    pub fn new(s: &str) -> (&str, Self) {
        let(s, op) = utils::extract_operator(s);
        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("At the Disco"),
        };

        (s, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let(s, lhs) = Number::new(s);
        let(s, _) = utils::extract_whitespace(s);

        let(s, op) = Op::new(s);
        let(s, _) = utils::extract_whitespace(s);

        let(s, rhs) = Number::new(s);
        (s, Self {lhs, rhs, op})
    }

    pub(crate) fn eval(&self) -> Val {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;

        let result = match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Div => lhs / rhs,
            Op::Mul => lhs * rhs,
        };

        Val::Number(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(Op::new("+"), ("", Op::Add));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(Op::new("-"), ("", Op::Sub));
    }

    #[test]
    fn parse_multi() {
        assert_eq!(Op::new("*"), ("", Op::Mul));

    }

    #[test]
    fn parse_division() {
        assert_eq!(Op::new("/"), ("", Op::Div));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            (
                "",
                Expr {
                    lhs:Number(1),
                    rhs:Number(2),
                    op: Op::Add,
                }
            )

        );
    }

    #[test]
    fn expr_extract_w_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            (
                "",
                Expr {
                    lhs:Number(2),
                    rhs:Number(2),
                    op: Op::Mul,
                }
            )
        )
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr{
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Add,
            }
            .eval(),
            Val::Number(20),
        );
    }
    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr{
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Sub,
            }
                .eval(),
            Val::Number(0),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr{
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Mul,
            }
                .eval(),
            Val::Number(100),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr{
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Div,
            }
                .eval(),
            Val::Number(1),
        );
    }
}



