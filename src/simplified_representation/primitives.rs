use crate::{parser::lexer::SourcePosition, Field, FieldList};

/// Enum representing the different kinds of identifiers in the intermediate representation.
#[derive(Debug, Clone, PartialEq)]
pub enum IrIdentifierKind {
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

/// Struct representing an identifier in the intermediate representation.
#[derive(Debug, Clone, PartialEq)]
pub struct IrIdentifier {
    pub unresolved: String,
    pub resolved: Option<String>,
    pub type_reference: Option<String>,
    pub kind: IrIdentifierKind,
    pub is_definition: bool,
    pub source_location: (SourcePosition, SourcePosition),
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

impl From<IrIdentifier> for SrType {
    fn from(value: IrIdentifier) -> Self {
        Self {
            main_type: value.unresolved,
            sub_types: vec![],
        }
    }
}

impl IrIdentifier {
    /// Constructor for the IrIdentifier struct.
    pub fn new(
        unresolved: String,
        kind: IrIdentifierKind,
        source_location: (SourcePosition, SourcePosition),
    ) -> Self {
        Self {
            unresolved,
            resolved: None,
            type_reference: None,
            kind,
            is_definition: false,
            source_location,
        }
    }

    /// Method to get the qualified name of the identifier.
    pub fn qualified_name(&self) -> Result<String, String> {
        // TODO: Change to resolved or throw
        if let Some(resolved) = &self.resolved {
            Ok(resolved.clone())
        } else {
            Ok(format!("[{}]", self.unresolved).to_string())
        }
    }
}

/// Struct representing an enum value in the intermediate representation.
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: IrIdentifier,
    pub id: u64,
    pub data: Option<IrIdentifier>,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl EnumValue {
    /// Constructor for the EnumValue struct.
    pub fn new(name: IrIdentifier, data: Option<IrIdentifier>) -> Self {
        Self { name, id: 0, data }
    }
    /// Method to set the id of the enum value.
    pub fn set_id(&mut self, v: u64) {
        self.id = v
    }
}

/// Struct representing a tuple in the intermediate representation.
#[derive(Debug, Clone)]
pub struct Tuple {
    pub fields: Vec<IrIdentifier>,
    // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl Tuple {
    /// Constructor for the Tuple struct.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Method to add a field to the tuple.
    pub fn add_field(&mut self, value: IrIdentifier) {
        self.fields.push(value);
    }
}

/// Struct representing a variant in the intermediate representation.
#[derive(Debug, Clone)]
pub struct Variant {
    pub fields: Vec<EnumValue>, // (name, id, data)
                                // TODO:     pub source_location: (SourcePosition,SourcePosition)
}

impl Variant {
    /// Constructor for the Variant struct.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Method to add a field to the variant.
    pub fn add_field(&mut self, field: EnumValue) {
        let id: u64 = match self.fields.last() {
            // if we have at least one field, use the id of the last field + 1
            Some(enum_value) => enum_value.id + 1,
            // else this is the first field, so use 0
            None => 0,
        };
        let mut field = field.clone();
        field.set_id(id);
        self.fields.push(field);
    }
}

/// Enum representing the different kinds of concrete types in the intermediate representation.
#[derive(Debug, Clone)]
pub enum ConcreteType {
    Tuple {
        name: IrIdentifier,
        namespace: IrIdentifier,
        data_layout: Box<Tuple>,
    },
    Variant {
        name: IrIdentifier,
        namespace: IrIdentifier,
        data_layout: Box<Variant>,
    },
}

/// Enum representing the different kinds of functions in the intermediate representation.
#[derive(Debug, Clone)]
pub enum FunctionKind {
    Procedure,
    Transition,
    Function,
}

/// Struct representing a concrete function in the intermediate representation.
#[derive(Debug, Clone)]
pub struct ConcreteFunction {
    pub name: IrIdentifier,
    pub namespace: IrIdentifier,
    pub function_kind: FunctionKind,
    pub return_type: Option<String>, // TODO: Should be Identifier
    pub arguments: FieldList,
}

/// Struct representing a contract field in the intermediate representation.
#[derive(Debug)]
pub struct ContractField {
    pub namespace: IrIdentifier,
    pub variable: Field,
}

/// Struct representing the intermediate representation of a program.
#[derive(Debug, Default)]
pub struct IntermediateRepresentation {
    pub name: String,
    pub version: String,
    pub init_params: FieldList,
    pub type_definitions: Vec<ConcreteType>,
    pub function_definitions: Vec<ConcreteFunction>,
    pub fields_definitions: Vec<ContractField>,
}
