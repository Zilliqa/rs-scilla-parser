use crate::{simplified_representation::primitives::ContractField, Type};

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub r#type: Type,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            name: Default::default(),
            r#type: Type::Other(Default::default()),
        }
    }
}

impl Field {
    /// Creates a new instance of field.
    ///
    /// Arguments:
    ///
    /// * `name`: A string representing the name of the field.
    /// * `r#type`: Type of the field.
    pub fn new(name: &str, r#type: Type) -> Self {
        Self {
            name: name.to_string(),
            r#type,
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct FieldList(pub Vec<Field>);

impl std::ops::Deref for FieldList {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<ContractField>> for FieldList {
    fn from(fields: Vec<ContractField>) -> Self {
        Self(fields.into_iter().map(|f| f.variable).collect())
    }
}

impl FieldList {
    pub fn push(&mut self, field: Field) {
        self.0.push(field);
    }
}
