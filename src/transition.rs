use crate::FieldList;

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

impl std::ops::DerefMut for TransitionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
