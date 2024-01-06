/// Enum representing the different kinds of identifiers in the simplified representation.
#[derive(Debug, Clone, PartialEq)]
pub enum SrIdentifierKind {
    FunctionName,
    StaticFunctionName,
    TransitionName,
    ProcedureName,
    TemplateFunctionName,
    ExternalFunctionName,

    TypeName,
    ComponentName,
    Event,
    Namespace,
    BlockLabel,

    ContextResource,

    // Storage and reference
    VirtualRegister,
    VirtualRegisterIntermediate,
    Memory,
    State,

    // More info needed to derive kind
    Unknown,
}

/// Struct representing an identifier in the simplified representation.
#[derive(Debug, Clone, PartialEq)]
pub struct SrIdentifier {
    pub unresolved: String,
    pub resolved: Option<String>,
    pub type_reference: Option<String>,
    pub kind: SrIdentifierKind,
    pub is_definition: bool,
}

#[derive(Debug, Clone)]
pub struct SrType {
    pub main_type: String,
    pub sub_types: Vec<SrType>,
}

impl SrType {
    pub fn push_sub_type(&mut self, sub_type: SrType) {
        self.sub_types.push(sub_type);
    }
}

impl From<SrIdentifier> for SrType {
    fn from(value: SrIdentifier) -> Self {
        Self {
            main_type: value.unresolved,
            sub_types: vec![],
        }
    }
}

impl SrIdentifier {
    pub fn new(unresolved: String, kind: SrIdentifierKind) -> Self {
        Self {
            unresolved,
            resolved: None,
            type_reference: None,
            kind,
            is_definition: false,
        }
    }

    pub fn qualified_name(&self) -> Result<String, String> {
        // TODO: Change to resolved or throw
        if let Some(resolved) = &self.resolved {
            Ok(resolved.clone())
        } else {
            Ok(format!("[{}]", self.unresolved).to_string())
        }
    }
}
