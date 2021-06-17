use crate::stmt::Stmt;
use crate::utils;
use crate::env::Env;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub(crate) stmts: Vec<Stmt>,
}

impl Block {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let(s, _) = utils::extract_whitespace(s);
        
        let (s, stmts) = utils::sequence(Stmt::new, s)?;

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block {stmts}))
    }

    pub fn eval(&self, env: &Env) -> Result<Val, String> {
        if self.stmts.is_empty() {
            return Ok(Val::Unit);
        }

        let mut child_env = env.create_child();

        let stmts_except_last = &self.stmts[..self.stmts.len() - 1];
        for stmt in stmts_except_last {
            stmt.eval(&mut child_env)?;
        }

        self.stmts.last().unwrap().eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Expr, Number, BindingUsage, Op};
    use super::*;
    use crate::binding_def::BindingDef;

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
                "", 
                Block {
                    stmts:vec![Stmt::Expr(Expr::Number(Number(5)))],
                },
            )),
        );
    }
    #[test]
    fn parse_multival_block() {
        assert_eq!(
            Block::new(
                "{
                    let a = 5
                    let b = a
                    b
                }",
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(5))
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string(),
                            }),
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name:"b".to_string(),
                        })),
                    ],
                },
            )),
        );
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block {stmts: Vec::new()}.eval(&Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_block_one_expr() {
        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(Expr::Number(Number(25)))],
            }
        .eval(&Env::default()),
        Ok(Val::Number(25)),
        );
    }

    #[test]
    fn eval_block_binding_def_and_use() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name:"one".to_string(),
                        val: Expr::Number(Number(1)),
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name:"one".to_string(),
                    })),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Number(1)),
        );
    }

    #[test]
    fn eval_block_with_many_binds() {
        assert_eq!(
            Block{
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        val: Expr::Number(Number(10)),
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        val: Expr::Number(Number(10)),
                    }),
                    Stmt::BindingDef(BindingDef {
                        name: "test".to_string(),
                        val: Expr::Number(Number(10)),
                    }),

                ],
            }
            .eval(&Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_block_many_expr() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Expr(Expr::Number(Number(10))),
                    Stmt::Expr(Expr::Number(Number(10))),
                    Stmt::Expr(Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(10),
                        op: Op::Sub,
                    }),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Number(0)),
        );
    }
    
    #[test]
    fn eval_block_binding_from_parent() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        val: Expr::BindingUsage(BindingUsage {
                            name: "foo".to_string(),
                        }),
                    }),
                    Stmt::Expr(Expr::BindingUsage(BindingUsage {
                        name:"bar".to_string(),
                    })),
                ],
            }
            .eval(&env),
            Ok(Val::Number(10)),
        );
    }
}