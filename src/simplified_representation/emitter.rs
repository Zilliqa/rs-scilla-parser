use crate::{
    ast::{converting::AstConverting, nodes::*, visitor::AstVisitor},
    parser::lexer::SourcePosition,
    simplified_representation::primitives::*,
    Contract, Field, FieldList, Transition,
};

use crate::ast::{TraversalResult, TreeTraversalMode};

#[derive(Debug, Clone)]
enum StackObject {
    IrIdentifier(SrIdentifier),
    VariableDeclaration(Field),
    TypeDefinition(SrType),
}

/// The `SrEmitter` struct is used for bookkeeping during the conversion of a Scilla AST to a simplified representation.
/// It implements the `AstConverting` trait, which is a generic trait for AST conversions.
#[derive(Default)]
pub struct SrEmitter {
    stack: Vec<StackObject>,
    contract: Contract,
}

impl SrEmitter {
    fn pop_ir_identifier(&mut self) -> Result<SrIdentifier, String> {
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
    pub fn emit(mut self, node: &NodeProgram) -> Result<Contract, String> {
        node.contract_definition.visit(&mut self)?;
        Ok(self.contract)
    }
}

impl AstConverting for SrEmitter {
    fn push_source_position(&mut self, _start: &SourcePosition, _end: &SourcePosition) {}

    fn pop_source_position(&mut self) {}

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
        match mode {
            TreeTraversalMode::Enter => match node {
                NodeTypeNameIdentifier::ByteStringType(bytestr) => {
                    let symbol = SrIdentifier::new(bytestr.to_string(), SrIdentifierKind::Unknown);

                    self.stack.push(StackObject::IrIdentifier(symbol));
                }
                NodeTypeNameIdentifier::EventType => {}
                NodeTypeNameIdentifier::TypeOrEnumLikeIdentifier(name) => {
                    let symbol = SrIdentifier::new(name.to_string(), SrIdentifierKind::Unknown);

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
        node: &NodeMetaIdentifier,
    ) -> Result<TraversalResult, String> {
        match node {
            NodeMetaIdentifier::ByteString => {
                let symbol = SrIdentifier::new("ByStr".to_string(), SrIdentifierKind::Unknown);
                self.stack.push(StackObject::IrIdentifier(symbol));
                Ok(TraversalResult::SkipChildren)
            }
            _ => Ok(TraversalResult::Continue),
        }
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
                            address_type: None,
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
                            address_type: None,
                        };
                        self.stack.push(StackObject::TypeDefinition(map));
                    }
                    NodeTypeMapValue::MapValueAddressType(value) => {
                        value.visit(self)?;
                        let value = self.pop_type_definition()?;
                        let key = self.pop_ir_identifier()?;
                        let map = SrType {
                            main_type: "Map".to_string(),
                            sub_types: vec![key.into(), value],
                            address_type: None,
                        };
                        self.stack.push(StackObject::TypeDefinition(map));
                    }
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
            NodeTypeArgument::EnclosedTypeArgument(t) => {
                let _ = t.visit(self)?;
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
                if !args.is_empty() {
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
            }
            NodeScillaType::FunctionType(_a, _b) => {
                unimplemented!()
            }

            NodeScillaType::PolyFunctionType(_name, _a) => {
                unimplemented!()
            }
            NodeScillaType::EnclosedType(a) => {
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::ScillaAddresseType(a) => {
                let _ = (*a).visit(self)?;
            }
            NodeScillaType::TypeVarType(_name) => {
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
        mode: TreeTraversalMode,
        node: &NodeAddressTypeField,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                if let NodeVariableIdentifier::VariableName(n) = &node.identifier.node {
                    node.type_name.visit(self)?;
                    let typename = self.pop_type_definition()?;
                    let s = StackObject::VariableDeclaration(Field::new(&n.node, typename.into()));
                    self.stack.push(s);
                }
            }
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_address_type(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeAddressType,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                node.identifier.visit(self)?;
                let identifier = self.pop_ir_identifier()?;
                self.stack
                    .push(StackObject::TypeDefinition(identifier.into()));
                let mut main_type = self.pop_type_definition()?;
                let mut fields = vec![];
                for field in &node.address_fields {
                    field.visit(self)?;
                    let field = self.pop_variable_declaration()?;
                    fields.push(field);
                }
                main_type.address_type = Some(AddressType {
                    type_name: node.type_name.node.clone(),
                    fields: FieldList(fields),
                });
                self.stack.push(StackObject::TypeDefinition(main_type));
            }
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::SkipChildren)
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
                self.stack.push(StackObject::IrIdentifier(SrIdentifier {
                    unresolved: name.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: SrIdentifierKind::ComponentName,
                    is_definition: false,
                }));
            }
            NodeComponentId::WithTypeLikeName(name) => {
                self.stack.push(StackObject::IrIdentifier(SrIdentifier {
                    unresolved: name.to_string(),
                    resolved: None,
                    type_reference: None,
                    kind: SrIdentifierKind::ComponentName,
                    is_definition: false,
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
                    self.contract.init_params.push(init_param);
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
        // Deliberate pass through
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
    }

    fn emit_program(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeProgram,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn emit_library_definition(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeLibraryDefinition,
    ) -> Result<TraversalResult, String> {
        unimplemented!()
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
        let _ = node.contract_name.visit(self)?;
        self.contract.name = node.contract_name.to_string();

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

        Ok(TraversalResult::SkipChildren)
    }

    fn emit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        node: &NodeContractField,
    ) -> Result<TraversalResult, String> {
        let _ = node.typed_identifier.visit(self)?;

        let field = self.pop_variable_declaration()?;
        let _ = node.right_hand_side.visit(self)?;

        self.contract.fields.push(field);

        Ok(TraversalResult::SkipChildren)
    }
    fn emit_with_constraint(
        &mut self,
        _mode: TreeTraversalMode,
        _node: &NodeWithConstraint,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
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

        let arguments = node
            .parameters
            .node
            .parameters
            .iter()
            .map(|arg| {
                let _ = arg.visit(self)?;
                self.pop_variable_declaration()
            })
            .collect::<Result<Vec<Field>, _>>()?;

        let mut function_name = self.pop_ir_identifier()?;
        assert!(function_name.kind == SrIdentifierKind::ComponentName);
        function_name.kind = SrIdentifierKind::TransitionName;
        function_name.is_definition = true;

        self.contract.transitions.push(Transition::new(
            &function_name.unresolved,
            FieldList(arguments),
        ));

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
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueArguments,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                match node {
                    NodeTypeMapValueArguments::EnclosedTypeMapValue(_) => todo!(),
                    NodeTypeMapValueArguments::GenericMapValueArgument(g) => {
                        g.visit(self)?;
                        let identifier = self.pop_ir_identifier()?;
                        self.stack
                            .push(StackObject::TypeDefinition(identifier.into()));
                    }
                    NodeTypeMapValueArguments::MapKeyValueType(_, _) => todo!(),
                };
            }
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::SkipChildren)
    }
    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        mode: TreeTraversalMode,
        node: &NodeTypeMapValueAllowingTypeArguments,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                match node {
                    NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(m) => {
                        m.visit(self)?;
                    }
                    NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(m, args) => {
                        m.visit(self)?;
                        let identifier = self.pop_ir_identifier()?;
                        self.stack
                            .push(StackObject::TypeDefinition(identifier.into()));
                        if !args.is_empty() {
                            let mut main_type = self.pop_type_definition()?;
                            for arg in args {
                                let _ = arg.visit(self)?;
                                let sub_type = self.pop_type_definition()?;
                                main_type.push_sub_type(sub_type);
                            }
                            self.stack.push(StackObject::TypeDefinition(main_type));
                        }
                    }
                };
            }
            TreeTraversalMode::Exit => (),
        }
        Ok(TraversalResult::SkipChildren)
    }
}
