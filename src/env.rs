use std::collections::HashMap;
use crate::val::Val;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env<'parent> {
    bindings: HashMap<String, Val>,
    parent:Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.get_binding_value_without_error(name)
        .ok_or_else(|| format!("binding with name '{}' doesn't exist", name))
    }

    pub(crate) fn get_binding_value_without_error(&self, name: &str) -> Option<Val> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.get_binding_value_without_error(name))
        })
    }

    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }
}