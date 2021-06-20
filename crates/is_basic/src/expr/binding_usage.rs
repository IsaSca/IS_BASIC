use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;

        Ok((
            s, 
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name:"abc".to_string(),
                },
            )),
        );
    }

    #[test]
    fn eval_existing_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name:"foo".to_string(),
            }
            .eval(&env),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn eval_no_existing_binding_use() {
        let empty = Env::default();

        assert_eq!(
            BindingUsage {
                name: "I don't think, or exist".to_string(),
            }
            .eval(&empty),
            Err("binding with name 'I don't think, or exist' doesn't exist".to_string())
        )
    }
}