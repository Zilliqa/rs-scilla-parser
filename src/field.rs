use crate::{
    intermediate_representation::primitives::{ContractField, VariableDeclaration},
    Type,
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Default)]
pub struct FieldList(pub Vec<Field>);

impl std::ops::Deref for FieldList {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<ContractField>> for FieldList {
    fn from(value: Vec<ContractField>) -> Self {
        println!("{value:?}");
        FieldList::default()
    }
}

impl From<Vec<VariableDeclaration>> for FieldList {
    fn from(variables: Vec<VariableDeclaration>) -> Self {
        Self(
            variables
                .into_iter()
                .map(|v| Field {
                    name: v.name.unresolved,
                    r#type: v.typename.into(),
                })
                .collect(),
        )
    }
}
