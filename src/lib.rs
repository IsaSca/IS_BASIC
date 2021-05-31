mod utils;
#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
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
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Bad Operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> Self {
        let lhs = Number::new(s);
        let rhs = Number::new(s);
        let op = Op::new(s);

        Self {lhs, rhs, op}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"),Number(123));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(Op::new("-"), Op::Sub);
    }

    #[test]
    fn parse_multi() {
        assert_eq!(Op::new("*"), Op::Mul);

    }

    #[test]
    fn parse_division() {
        assert_eq!(Op::new("/"), Op::Div)
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Expr {
                lhs: Number(1),
                rhs: Number(2),
                op: Op::Add,
            },
        );
    }
}


