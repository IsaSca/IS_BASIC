use std::collections::HashMap;
use crate::val::Val;
use crate::stmt::Stmt;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    named: HashMap<String, NamedInfo>,
    parent:Option<&'parent Self>,
}

#[derive(Debug, ParitalEq)]
enum NamedInfo {
    Binding(Val),
    Func { params: Vec<String>, body: Stmt},
}

impl<'parent> Env<'parent> {
    pub(crate) fn store_binding(&mut self, name: String, val: Val) {

        self.named.insert(name, NamedInfo::Binding(val));
    }

    pub(crate) fn store_func(&mut self, name: String, params: Vec<String>, body: Stmt) {
        self.named.insert(name, NamedInfo::Func {params, body});
    }

    pub fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.get_binding_value_without_error(name)
        .ok_or_else(|| format!("binding with name '{}' doesn't exist", name))
    }

     fn get_binding_value_without_error(&self, name: &str) -> Option<Val> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.get_binding_value_without_error(name))
        })
    }

    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            named: HashMap::new(),
            parent: Some(self),
        }
    }
}