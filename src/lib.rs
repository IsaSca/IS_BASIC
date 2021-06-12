pub mod expr;
pub mod binding_def;
pub mod val;

pub mod stmt;

mod utils;
mod env;

pub struct Parse(stmt::Stmt);

pub fn parse(s: &str)-> Result<?, String> {
    let(s, stmt) = stmt::Stmt::new(s)?;

    if s.is_empty() {
        Ok(stmt)
    } else {
        Err("Input was not fully consumed".to_string())
    }
}

