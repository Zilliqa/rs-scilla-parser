use std::mem;

use crate::{
    ast::{converting::AstConverting, nodes::*, visitor::AstVisitor},
    parser::lexer::SourcePosition,
    simplified_representation::primitives::*,
    Field, FieldList,
};

use crate::ast::{TraversalResult, TreeTraversalMode};

#[derive(Debug, Clone)]
enum StackObject {
    IrIdentifier(IrIdentifier),
    VariableDeclaration(Field),
    TypeDefinition(SrType),
}

/// The `SrEmitter` struct is used for bookkeeping during the conversion of a Scilla AST to a simplified representation.
/// It implements the `AstConverting` trait, which is a generic trait for AST conversions.
pub struct SrEmitter {
    /// Stack of objects used during the conversion process.
    stack: Vec<StackObject>,

    /// Current namespace being processed.
    current_namespace: IrIdentifier,

    /// Stack of namespaces used during the conversion process.
    namespace_stack: Vec<IrIdentifier>,

    /// Intermediate representation of the AST.
    ir: Box<IntermediateRepresentation>,

    /// Source positions of the AST nodes.
    source_positions: Vec<(SourcePosition, SourcePosition)>,
}

impl SrEmitter {
    pub fn new() -> Self {
        let ns = IrIdentifier {
            unresolved: "".to_string(),
            resolved: None,
            type_reference: None,
            kind: IrIdentifierKind::Namespace,
            is_definition: false,
            source_location: (
                SourcePosition::start_position(),
                SourcePosition::start_position(),
            ),
        };
        // TODO: Repeat similar code for all literals
        SrEmitter {
            stack: Vec::new(),
            current_namespace: ns.clone(),
            namespace_stack: [ns].to_vec(),
            ir: Box::new(IntermediateRepresentation::default()),
            source_positions: [(
                SourcePosition::invalid_position(),
                SourcePosition::invalid_position(),
            )]
            .to_vec(), // TODO: this should not be necessary
        }
    }

    fn current_location(&self) -> (SourcePosition, SourcePosition) {
        self.source_positions
            .last()
            .expect("Unable to determine source location")
            .clone()
    }

    fn push_namespace(&mut self, mut ns: IrIdentifier) {
        // TODO: Update ns to use nested namespaces
        ns.kind = IrIdentifierKind::Namespace;
        self.namespace_stack.push(ns.clone());
        self.current_namespace = ns;
    }

    fn pop_namespace(&mut self) {
        self.namespace_stack.pop();
        if let Some(ns) = self.namespace_stack.last() {
            self.current_namespace = ns.clone();
        } else {
            panic!("Namespace stack is empty.");
        }
    }

    fn pop_ir_identifier(&mut self) -> Result<IrIdentifier, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::IrIdentifier(n) => n,
                _ => {
                    return Err(format!("Expected symbol name, but found {:?}.", candidate));
                }
            }
        } else {
            return Err("Expected symbol name, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_variable_declaration(&mut self) -> Result<Field, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::VariableDeclaration(n) => n,
                _ => {
                    return Err(format!(
                        "Expected variable declaration, but found {:?}.",
                        candidate
                    ));
                }
            }
        } else {
            return Err("Expected variable declaration, but found nothing.".to_string());
        };

        Ok(ret)
    }

    fn pop_type_definition(&mut self) -> Result<SrType, String> {
        let ret = if let Some(candidate) = self.stack.pop() {
            match candidate {
                StackObject::TypeDefinition(n) => n,
                _ => {
                    return Err(format!(
                        "Expected type definition, but found {:?}.",
                        candidate
                    ));
                }
            }
        } else {
            return Err("Expected type definition, but found nothing.".to_string());
        };

        Ok(ret)
    }
    pub fn emit(&mut self, node: &NodeProgram) -> Result<Box<IntermediateRepresentation>, String> {
        let result = node.contract_definition.visit(self);
        match result {
            Err(m) => panic!("{}", m),
            _ => (),
        }

        // Creating type table

        // Annotating symbols with types

        // Returning
        let mut ret = Box::new(IntermediateRepresentation::default());
        mem::swap(&mut self.ir, &mut ret);

        Ok(ret)
    }
}

impl AstConverting for SrEmitter {
    fn push_source_position(&mut self, start: &SourcePosition, end: &SourcePosition) {
        self.source_positions.push((start.clone(), end.clone()));
    }

    fn pop_source_position(&mut self) {
        self.source_positions.pop();
    }

    fn emit_byte_str(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeByteStr,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
    fn emit_type_name_identifier(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeNameIdentifier,
    ) -> Result<TraversalResult, String> {
        println!("{node:#?}");
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeTypeNameIdentifier::ByteStringType(bytestr) => {
                    let symbol = IrIdentifier::new(
                        bytestr.to_string(),
                        IrIdentifierKind::Unknown,
                        self.current_location(),
                    );

                    self.stack.push(StackObject::IrIdentifier(symbol));
                }
                NodeTypeNameIdentifier::EventType => {}
                NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier(name) => {
                    let symbol = IrIdentifier::new(
                        name.to_string(),
                        IrIdentifierKind::Unknown,
                        self.current_location(),
                    );

                    self.stack.push(StackObject::IrIdentifier(symbol));
                }
            },
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::Continue)
    }
    fn emit_imported_name(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeImportedName,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_import_declarations(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeImportDeclarations,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_meta_identifier(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeMetaIdentifier,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
    fn emit_variable_identifier(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeVariableIdentifier,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_builtin_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeBuiltinArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_key(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypeMapKey,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeMapKey::GenericMapKey(key) => key.visit(self)?,
            NodeTypeMapKey::EnclosedGenericId(key) => key.visit(self)?,
            NodeTypeMapKey::EnclosedAddressMapKeyType(key) => key.visit(self)?,
            NodeTypeMapKey::AddressMapKeyType(key) => key.visit(self)?,
        };
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_map_value(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValue,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                match node {
                    NodeTypeMapValue::MapValueTypeOrEnumLikeIdentifier(value) => {
                        value.visit(self)?;
                        let value = self.pop_ir_identifier()?;
                        let key = self.pop_ir_identifier()?;
                        let map = SrType {
                            main_type: "Map".to_string(),
                            sub_types: vec![key.into(), value.into()],
                        };
                        self.stack.push(StackObject::TypeDefinition(map));
                    }
                    NodeTypeMapValue::MapKeyValue(value) => {
                        value.visit(self)?;
                    }
                    NodeTypeMapValue::MapValueParenthesizedType(value) => {
                        value.visit(self)?;
                        let value = self.pop_type_definition()?;
                        let key = self.pop_ir_identifier()?;
                        let map = SrType {
                            main_type: "Map".to_string(),
                            sub_types: vec![key.into(), value],
                        };
                        self.stack.push(StackObject::TypeDefinition(map));
                    }
                    NodeTypeMapValue::MapValueAddressType(_value) => unimplemented!(),
                };
            }
            TreeTraversalMode::Exit => {}
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_argument(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypeArgument,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeTypeArgument::EnclosedTypeArgument(_) => {
                unimplemented!();
            }
            NodeTypeArgument::GenericTypeArgument(n) => {
                let _ = n.visit(self)?;
                let identifier = self.pop_ir_identifier()?;
                self.stack
                    .push(StackObject::TypeDefinition(identifier.into()));
            }
            NodeTypeArgument::TemplateTypeArgument(_) => {
                unimplemented!();
            }
            NodeTypeArgument::AddressTypeArgument(_) => {
                unimplemented!();
            }
            NodeTypeArgument::MapTypeArgument(_, _) => {
                unimplemented!();
            }
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_scilla_type(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeScillaType,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeScillaType::GenericTypeWithArgs(lead, args) => {
                let _ = lead.visit(self)?;
                let identifier = self.pop_ir_identifier()?;
                self.stack
                    .push(StackObject::TypeDefinition(identifier.into()));
                if args.len() > 0 {
                    let mut main_type = self.pop_type_definition()?;
                    for arg in args {
                        let _ = arg.visit(self)?;
                        let sub_type = self.pop_type_definition()?;
                        main_type.push_sub_type(sub_type);
                    }
                    self.stack.push(StackObject::TypeDefinition(main_type));
                }
            }
            NodeScillaType::MapType(key, value) => {
                let _ = key.visit(self)?;
                let _ = value.visit(self)?;
                // let value = self.pop_type_definition()?;
                // let key = self.pop_type_definition()?;
                // let map = SrType {
                //     main_type: "Map".to_string(),
                //     sub_types: vec![key, value],
                // };
                // self.stack.push(StackObject::TypeDefinition(map));
            }
            NodeScillaType::FunctionType(a, b) => {
                let _ = (*a).visit(self)?;
                let _ = (*b).visit(self)?;
                // TODO: Implement the function type
                unimplemented!()
            }

            NodeScillaType::PolyFunctionType(_name, a) => {
                // TODO: What to do with name
                let _ = (*a).visit(self)?;
                unimplemented!()
            }
            NodeScillaType::EnclosedType(a) => {
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::ScillaAddresseType(a) => {
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::TypeVarType(_name) => {
                /*
                self.stack
                    .push(StackObject::Identifier(Identifier::TypeName(
                        name.to_string(),
                    )));
                    */
                unimplemented!()
            }
        };
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_map_entry(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapEntry,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
    fn emit_address_type_field(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeAddressTypeField,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_address_type(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeAddressType,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_full_expression(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeFullExpression,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_message_entry(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeMessageEntry,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern_match_expression_clause(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodePatternMatchExpressionClause,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_atomic_expression(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeAtomicExpression,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_contract_type_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeContractTypeArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_value_literal(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeValueLiteral,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_map_access(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeMapAccess,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodePattern,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_argument_pattern(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeArgumentPattern,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_pattern_match_clause(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodePatternMatchClause,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_blockchain_fetch_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeBlockchainFetchArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }

    fn emit_statement(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeStatement,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_remote_fetch_statement(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeRemoteFetchStatement,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_component_id(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeComponentId,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeComponentId::WithRegularId(name) => {
                self.stack.push(StackObject::IrIdentifier(IrIdentifier {
                    unresolved: name.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: IrIdentifierKind::ComponentName,
                    is_definition: false,
                    source_location: self.current_location(),
                }));
            }
            NodeComponentId::WithTypeLikeName(name) => {
                self.stack.push(StackObject::IrIdentifier(IrIdentifier {
                    unresolved: name.to_string(), // TODO: Travese the tree first and then construct the name
                    resolved: None,
                    type_reference: None,
                    kind: IrIdentifierKind::ComponentName,
                    is_definition: false,
                    source_location: self.current_location(),
                }));
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_component_parameters(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeComponentParameters,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                for param in node.parameters.iter() {
                    let _ = param.visit(self)?;
                    let init_param = self.pop_variable_declaration()?;
                    self.ir.init_params.push(init_param);
                }
            }
            TreeTraversalMode::Exit => {}
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_parameter_pair(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeParameterPair,
    ) -> Result<TraversalResult, String> {
        // Delibarate pass through
        Ok(TraversalResult::Continue)
    }

    fn emit_component_body(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeComponentBody,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_statement_block(
        &mut self,
        _node: TreeTraversalMode,
        _mode: &NodeStatementBlock,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
    fn emit_typed_identifier(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTypedIdentifier,
    ) -> Result<TraversalResult, String> {
        let name = node.identifier_name.clone();
        let _ = node.annotation.visit(self)?;

        let typename = self.pop_type_definition()?;

        let s = StackObject::VariableDeclaration(Field::new(&name.node, typename.into()));
        self.stack.push(s);

        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_annotation(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeAnnotation,
    ) -> Result<TraversalResult, String> {
        // Pass through
        Ok(TraversalResult::Continue)
        //        unimplemented!();
    }

    fn emit_program(
        &mut self,
        mode: TreeTraversalMode,
        _node: &NodeProgram,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                /*
                TODO: Move to LLVM emitter
                // Parse the version string to u64
                let version = match node.version.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Failed to parse version");
                        return Err("Scilla version must be an integer".to_string());
                    }
                };
                let node_version_value = self.context.i64_type().const_int(version, false);
                // Add a global constant named `scilla_version` to the module
                let addr_space = inkwell::AddressSpace::from(2u16);
                let scilla_version = self.module.add_global(
                    self.context.i64_type(),
                    Some(addr_space),
                    "scilla_version",
                );
                scilla_version.set_initializer(&node_version_value);
                scilla_version.set_constant(true);
                */
            }
            TreeTraversalMode::Exit => {
                // Not sure on what's to be done during exit
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn emit_library_definition(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeLibraryDefinition,
    ) -> Result<TraversalResult, String> {
        let _ = node.name.visit(self)?;
        let mut ns = self.pop_ir_identifier()?;
        assert!(ns.kind == IrIdentifierKind::Unknown);
        ns.kind = IrIdentifierKind::Namespace;

        self.push_namespace(ns);
        for def in node.definitions.iter() {
            let _ = def.visit(self)?;
        }

        self.pop_namespace();
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_library_single_definition(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeLibrarySingleDefinition,
    ) -> Result<TraversalResult, String> {
        unimplemented!()
    }

    fn emit_contract_definition(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> Result<TraversalResult, String> {
        // TODO: Decide whether the namespace should be distinct
        let _ = node.contract_name.visit(self)?;
        self.ir.name = node.contract_name.to_string();
        let mut ns = self.pop_ir_identifier()?;
        assert!(ns.kind == IrIdentifierKind::Unknown);
        ns.kind = IrIdentifierKind::Namespace;

        self.push_namespace(ns);

        let _ = node.parameters.visit(self)?;

        if let Some(constraint) = &node.constraint {
            let _ = constraint.visit(self)?;
        }

        for field in node.fields.iter() {
            let _ = field.visit(self)?;
        }

        for component in node.components.iter() {
            let _ = component.visit(self)?;
        }

        self.pop_namespace();
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> Result<TraversalResult, String> {
        let _ = node.typed_identifier.visit(self)?;

        let variable = self.pop_variable_declaration()?;
        let _ = node.right_hand_side.visit(self)?;
        // variable.name.kind = IrIdentifierKind::State;

        let field = ContractField {
            namespace: self.current_namespace.clone(),
            variable,
        };

        self.ir.fields_definitions.push(field);

        Ok(TraversalResult::SkipChildren)
    }
    fn emit_with_constraint(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeWithConstraint,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_component_definition(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeComponentDefinition,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
    fn emit_procedure_definition(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeProcedureDefinition,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_transition_definition(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeTransitionDefinition,
    ) -> Result<TraversalResult, String> {
        // Enter
        let _ = node.name.visit(self)?;

        let mut arguments = FieldList::default();
        for arg in node.parameters.node.parameters.iter() {
            let _ = arg.visit(self)?;
            let ir_arg = self.pop_variable_declaration()?;
            arguments.push(ir_arg);
        }

        let mut function_name = self.pop_ir_identifier()?;
        assert!(function_name.kind == IrIdentifierKind::ComponentName);
        function_name.kind = IrIdentifierKind::TransitionName;
        function_name.is_definition = true;

        let function = ConcreteFunction {
            name: function_name,
            namespace: self.current_namespace.clone(),
            function_kind: FunctionKind::Transition,
            return_type: None, // TODO: Pop of the stack
            arguments,
        };

        self.ir.function_definitions.push(function);

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_type_alternative_clause(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeAlternativeClause,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_map_value_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapValueArguments,
    ) -> Result<TraversalResult, String> {
        unimplemented!();
    }
    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
}
