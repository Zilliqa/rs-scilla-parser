use crate::{
    intermediate_representation::primitives::{ConcreteFunction, FunctionKind},
    FieldList,
};

#[derive(Debug, PartialEq)]
pub struct Transition {
    pub name: String,
    pub params: FieldList,
}

impl Transition {
    pub fn new(name: &str, params: FieldList) -> Self {
        Self {
            name: name.to_string(),
            params,
        }
    }

    pub fn new_without_param(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: FieldList::default(),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct TransitionList(pub Vec<Transition>);

impl std::ops::Deref for TransitionList {
    type Target = Vec<Transition>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<ConcreteFunction>> for TransitionList {
    fn from(function_definitions: Vec<ConcreteFunction>) -> Self {
        Self(
            function_definitions
                .into_iter()
                .filter_map(|f| match f.function_kind {
                    FunctionKind::Transition => Some(Transition {
                        name: f.name.unresolved,
                        params: f.arguments.into(),
                    }),
                    _ => None,
                })
                .collect(),
        )
    }
}
