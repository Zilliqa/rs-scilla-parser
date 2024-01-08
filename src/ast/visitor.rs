use crate::ast::{converting::AstConverting, nodes::*};

use super::{TraversalResult, TreeTraversalMode};

/// The `AstVisitor` trait is used for implementing the visiting behaviour for each AST node of the Scilla AST.
/// Each node in the AST implements this trait to define how it should be visited during the tree traversal.
/// The `visit` method is called with an `emitter` that implements the `AstConverting` trait, which is responsible for converting the AST to some other form.
/// The `visit` method returns a `Result` with a `TraversalResult` that informs the visitor algorithm how to proceed, or a `String` in case of an error.
pub trait AstVisitor {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String>;
}

impl<T: AstVisitor> AstVisitor for WithMetaData<T> {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        emitter.push_source_position(&self.start, &self.end);
        let ret = self.node.visit(emitter);
        emitter.pop_source_position();

        ret
    }
}

impl AstVisitor for NodeByteStr {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_byte_str(TreeTraversalMode::Enter, self)?;

        // No children

        match ret {
            TraversalResult::Continue => emitter.emit_byte_str(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeNameIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_name_identifier(TreeTraversalMode::Enter, self);

        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeNameIdentifier::ByteStringType(bs_type) => bs_type.visit(emitter),
                _ => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;

        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_type_name_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeImportedName {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_imported_name(TreeTraversalMode::Enter, self);

        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeImportedName::RegularImport(name) => name.visit(emitter),
                NodeImportedName::AliasedImport(name, alias) => {
                    name.visit(emitter)?;
                    alias.visit(emitter)
                }
            }
        } else {
            ret
        }?;

        match children_ret {
            TraversalResult::Continue => emitter.emit_imported_name(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeImportDeclarations {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_import_declarations(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for import in &self.import_list {
                import.visit(emitter)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_import_declarations(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeMetaIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_meta_identifier(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeMetaIdentifier::MetaName(name) => name.visit(emitter),
                NodeMetaIdentifier::MetaNameInNamespace(name, ns) => {
                    name.visit(emitter)?;
                    ns.visit(emitter)
                }
                NodeMetaIdentifier::MetaNameInHexspace(_, name) => name.visit(emitter),
                NodeMetaIdentifier::ByteString => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_meta_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeVariableIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_variable_identifier(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeVariableIdentifier::VariableInNamespace(type_name_identifier, _) => {
                    type_name_identifier.visit(emitter)
                }
                // Since VariableName and SpecialIdentifier don't have children
                // we can directly return ret here.
                _ => ret,
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_variable_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeBuiltinArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        // Call the code emitter at the entry of the NodeBuiltinArguments
        let ret = emitter.emit_builtin_arguments(TreeTraversalMode::Enter, self);
        // Call the visitor on all children of NodeBuiltinArguments if ret == Ok(TraversalResult::Continue)
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            // Visit each of the arguments
            self.arguments
                .iter()
                .map(|argument| argument.visit(emitter))
                .find(|r| *r == Err(String::from("Failure")))
                .unwrap_or(Ok(TraversalResult::Continue))
        } else {
            ret
        }?;
        // Call the code emitter at the exit of the NodeBuiltinArguments
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_builtin_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapKey {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_key(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapKey::GenericMapKey(node_met_id) => node_met_id.visit(emitter),
                NodeTypeMapKey::EnclosedGenericId(node_met_id) => node_met_id.visit(emitter),
                NodeTypeMapKey::EnclosedAddressMapKeyType(node_address_type) => {
                    node_address_type.visit(emitter)
                }
                NodeTypeMapKey::AddressMapKeyType(node_address_type) => {
                    node_address_type.visit(emitter)
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_map_key(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapValue {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_value(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapValue::MapValueTypeOrEnumLikeIdentifier(meta_id) => {
                    meta_id.visit(emitter)
                }
                NodeTypeMapValue::MapKeyValue(entry) => entry.visit(emitter),
                NodeTypeMapValue::MapValueParenthesizedType(value) => value.visit(emitter),
                NodeTypeMapValue::MapValueAddressType(address_type) => address_type.visit(emitter),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_map_value(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeArgument {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_argument(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeArgument::EnclosedTypeArgument(node) => node.visit(emitter),
                NodeTypeArgument::GenericTypeArgument(node) => node.visit(emitter),
                NodeTypeArgument::TemplateTypeArgument(_) => Ok(TraversalResult::Continue),
                NodeTypeArgument::AddressTypeArgument(node) => node.visit(emitter),
                NodeTypeArgument::MapTypeArgument(key_node, value_node) => {
                    match key_node.visit(emitter) {
                        Ok(TraversalResult::Continue) => value_node.visit(emitter),
                        Err(msg) => Err(msg),
                        _ => Ok(TraversalResult::Continue),
                    }
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_argument(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeScillaType {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_scilla_type(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeScillaType::GenericTypeWithArgs(id, args) => {
                    id.visit(emitter)?;
                    for arg in args {
                        arg.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeScillaType::MapType(key, value) => {
                    key.visit(emitter)?;
                    value.visit(emitter)
                }
                NodeScillaType::FunctionType(t1, t2) => {
                    t1.visit(emitter)?;
                    t2.visit(emitter)
                }
                NodeScillaType::PolyFunctionType(_, t) => t.visit(emitter),
                NodeScillaType::EnclosedType(t) => t.visit(emitter),
                NodeScillaType::ScillaAddresseType(t) => t.visit(emitter),
                NodeScillaType::TypeVarType(_) => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_scilla_type(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapEntry {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_entry(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.key.visit(emitter)?;
            self.value.visit(emitter)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_type_map_entry(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeAddressTypeField {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_address_type_field(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.identifier.visit(emitter)?;
            self.type_name.visit(emitter)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_address_type_field(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeAddressType {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_address_type(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.identifier.visit(emitter)?;

            for field in &self.address_fields {
                let ret = field.visit(emitter);

                ret.as_ref()?;
            }
            ret
        } else {
            Ok(TraversalResult::Continue)
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_address_type(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeFullExpression {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_full_expression(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeFullExpression::LocalVariableDeclaration {
                    expression,
                    containing_expression,
                    ..
                } => {
                    expression.visit(emitter)?;
                    containing_expression.visit(emitter)
                }
                NodeFullExpression::FunctionDeclaration { expression, .. } => {
                    expression.visit(emitter)
                }
                NodeFullExpression::FunctionCall {
                    function_name,
                    argument_list,
                    ..
                } => {
                    function_name.visit(emitter)?;
                    for arg in argument_list {
                        arg.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeFullExpression::ExpressionAtomic(atom_expr) => atom_expr.visit(emitter),
                NodeFullExpression::ExpressionBuiltin { xs, .. } => xs.visit(emitter),
                NodeFullExpression::Message(message_entries) => {
                    for entry in message_entries {
                        entry.visit(emitter)?;
                    }

                    Ok(TraversalResult::Continue)
                }
                NodeFullExpression::Match {
                    match_expression,
                    clauses,
                    ..
                } => {
                    match_expression.visit(emitter)?;
                    for clause in clauses {
                        clause.visit(emitter)?;
                    }

                    Ok(TraversalResult::Continue)
                }
                NodeFullExpression::ConstructorCall {
                    identifier_name,
                    argument_list,
                    ..
                } => {
                    identifier_name.visit(emitter)?;
                    for arg in argument_list {
                        arg.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeFullExpression::TemplateFunction { expression, .. } => {
                    expression.visit(emitter)
                }
                NodeFullExpression::TApp {
                    identifier_name,
                    type_arguments,
                    ..
                } => {
                    identifier_name.visit(emitter)?;
                    for targ in type_arguments {
                        targ.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_full_expression(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeMessageEntry {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_message_entry(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeMessageEntry::MessageLiteral(var_identifier, value_literal) => {
                    var_identifier.visit(emitter)?;
                    value_literal.visit(emitter)
                }
                NodeMessageEntry::MessageVariable(var_identifier1, var_identifier2) => {
                    var_identifier1.visit(emitter)?;
                    emitter.push_source_position(&var_identifier2.start, &var_identifier2.end);
                    var_identifier2.visit(emitter)
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_message_entry(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodePatternMatchExpressionClause {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_pattern_match_expression_clause(TreeTraversalMode::Enter, self);
        let pattern_ret = if ret == Ok(TraversalResult::Continue) {
            self.pattern.visit(emitter)
        } else {
            ret
        };
        let expression_ret = if pattern_ret == Ok(TraversalResult::Continue) {
            self.expression.visit(emitter)
        } else {
            pattern_ret
        }?;
        match expression_ret {
            TraversalResult::Continue => {
                emitter.emit_pattern_match_expression_clause(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeAtomicExpression {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_atomic_expression(TreeTraversalMode::Enter, self);
        // Only visit children if entering was successful and did not result in skipping
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeAtomicExpression::AtomicSid(sid) => sid.visit(emitter),
                NodeAtomicExpression::AtomicLit(lit) => lit.visit(emitter),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_atomic_expression(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeContractTypeArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_contract_type_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.type_arguments
                .iter()
                .map(|child| child.visit(emitter))
                .find(|result| *result == Err("".into()))
                .unwrap_or(Ok(TraversalResult::Continue))
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_contract_type_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeValueLiteral {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_value_literal(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeValueLiteral::LiteralInt(type_name, _) => type_name.visit(emitter),
                NodeValueLiteral::LiteralEmptyMap(type_map_key, type_map_value) => {
                    type_map_key.visit(emitter)?;
                    emitter.push_source_position(&type_map_value.start, &type_map_value.end);
                    type_map_value.visit(emitter)
                }
                _ => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_value_literal(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeMapAccess {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_map_access(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.identifier_name.visit(emitter)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_map_access(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodePattern {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_pattern(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodePattern::Wildcard => Ok(TraversalResult::Continue),
                NodePattern::Binder(_) => Ok(TraversalResult::Continue),
                NodePattern::Constructor(identifier, argument_patterns) => {
                    identifier.visit(emitter)?;
                    for pattern in argument_patterns {
                        pattern.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.emit_pattern(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeArgumentPattern {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_argument_pattern(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeArgumentPattern::WildcardArgument => Ok(TraversalResult::Continue),
                NodeArgumentPattern::BinderArgument(_) => Ok(TraversalResult::Continue),
                NodeArgumentPattern::ConstructorArgument(meta_identifier) => {
                    meta_identifier.visit(emitter)
                }
                NodeArgumentPattern::PatternArgument(pattern) => pattern.visit(emitter),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_argument_pattern(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodePatternMatchClause {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_pattern_match_clause(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            emitter
                .push_source_position(&self.pattern_expression.start, &self.pattern_expression.end);
            match self.pattern_expression.visit(emitter) {
                Err(msg) => Err(msg),
                _ => match &self.statement_block {
                    Some(stmt_block) => stmt_block.visit(emitter),
                    None => Ok(TraversalResult::Continue),
                },
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_pattern_match_clause(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeBlockchainFetchArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_blockchain_fetch_arguments(TreeTraversalMode::Enter, self);
        if let Ok(TraversalResult::Continue) = ret {
            // Visit each argument
            for arg in &self.arguments {
                arg.visit(emitter)?;
            }
        }
        match ret? {
            TraversalResult::Continue => {
                emitter.emit_blockchain_fetch_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeStatement {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_statement(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeStatement::Load {
                    right_hand_side, ..
                }
                | NodeStatement::Store {
                    right_hand_side, ..
                } => right_hand_side.visit(emitter),
                NodeStatement::RemoteFetch(statement) => statement.visit(emitter),
                NodeStatement::Bind {
                    right_hand_side, ..
                } => right_hand_side.visit(emitter),
                NodeStatement::ReadFromBC { arguments, .. } => {
                    if let Some(arg) = arguments {
                        arg.visit(emitter)
                    } else {
                        Ok(TraversalResult::Continue)
                    }
                }
                NodeStatement::MapGet { keys, .. }
                | NodeStatement::MapUpdate { keys, .. }
                | NodeStatement::MapGetExists { keys, .. }
                | NodeStatement::MapUpdateDelete { keys, .. } => {
                    for key in keys {
                        let ret = key.visit(emitter);

                        if ret != Ok(TraversalResult::Continue) {
                            return ret;
                        }
                    }

                    Ok(TraversalResult::Continue)
                }
                NodeStatement::Send {
                    identifier_name, ..
                }
                | NodeStatement::CreateEvnt {
                    identifier_name, ..
                } => identifier_name.visit(emitter),
                NodeStatement::Throw { error_variable, .. } => {
                    if let Some(variable) = error_variable {
                        variable.visit(emitter)
                    } else {
                        Ok(TraversalResult::Continue)
                    }
                }
                NodeStatement::MatchStmt {
                    variable, clauses, ..
                } => {
                    let ret = variable.visit(emitter);

                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }
                    for clause in clauses {
                        let ret = clause.visit(emitter);

                        if ret != Ok(TraversalResult::Continue) {
                            return ret;
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeStatement::CallProc {
                    component_id,
                    arguments,
                    ..
                } => {
                    let ret = component_id.visit(emitter);
                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }
                    for argument in arguments {
                        let ret = argument.visit(emitter);
                        if ret != Ok(TraversalResult::Continue) {
                            return ret;
                        }
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeStatement::Iterate {
                    identifier_name,
                    component_id,
                } => {
                    let ret = identifier_name.visit(emitter);
                    if ret != Ok(TraversalResult::Continue) {
                        return ret;
                    }

                    component_id.visit(emitter)
                }
                _ => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        };

        match children_ret? {
            TraversalResult::Continue => emitter.emit_statement(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeRemoteFetchStatement {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_remote_fetch_statement(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeRemoteFetchStatement::ReadStateMutable(_, _, variable) => {
                    variable.visit(emitter)
                }
                NodeRemoteFetchStatement::ReadStateMutableSpecialId(_, _, _) => {
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableMapAccess(_, _, _, accesses) => {
                    for access in accesses {
                        access.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableMapAccessExists(_, _, _, accesses) => {
                    for access in accesses {
                        access.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                NodeRemoteFetchStatement::ReadStateMutableCastAddress(_, variable, address) => {
                    variable.visit(emitter)?;
                    address.visit(emitter)
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_remote_fetch_statement(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeComponentId {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        // Emit enter event
        let ret = emitter.emit_component_id(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            // Handle child nodes
            match self {
                NodeComponentId::WithTypeLikeName(type_name_identifier) => {
                    type_name_identifier.visit(emitter)
                }
                NodeComponentId::WithRegularId(_) => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_component_id(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeComponentParameters {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_component_parameters(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for param in &self.parameters {
                param.visit(emitter)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_component_parameters(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeParameterPair {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_parameter_pair(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.identifier_with_type.visit(emitter)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_parameter_pair(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeComponentBody {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_component_body(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            if let Some(statement_block) = &self.statement_block {
                statement_block.visit(emitter)
            } else {
                Ok(TraversalResult::Continue)
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_component_body(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeStatementBlock {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_statement_block(TreeTraversalMode::Enter, self);
        // Visit each statement if not skipping children
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for statement in &self.statements {
                statement.visit(emitter)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_statement_block(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypedIdentifier {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_typed_identifier(TreeTraversalMode::Enter, self);
        // Visit the annotation child node if the enter phase didn't fail or skip children
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.annotation.visit(emitter)
        } else {
            ret
        };
        // Depending on the result of the children's visits, either fail or finish with the exit phase
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_typed_identifier(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeAnnotation {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_annotation(TreeTraversalMode::Enter, self);
        // Child element: self.type_name
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.type_name.visit(emitter)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_type_annotation(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeProgram {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        // Emit enter event
        let ret = emitter.emit_program(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            // Visit import_declarations if it's not None
            if let Some(import_declarations) = &self.import_declarations {
                let result = import_declarations.visit(emitter);

                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            // Visit library_definition if it's not None
            if let Some(library_definition) = &self.library_definition {
                let result = library_definition.visit(emitter);

                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            // Visit contract_definition

            self.contract_definition.visit(emitter)
        } else {
            ret
        };
        // Emit exit event
        match children_ret? {
            TraversalResult::Continue => emitter.emit_program(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeLibraryDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_library_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for definition in &self.definitions {
                let result = definition.visit(emitter);

                match result {
                    Err(msg) => return Err(msg),
                    _ => continue,
                };
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_library_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeLibrarySingleDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_library_single_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeLibrarySingleDefinition::LetDefinition {
                    variable_name: _,
                    type_annotation: _,
                    expression,
                } => {
                    // TODO: Unused variables aboce
                    let _ = expression.visit(emitter)?;

                    unimplemented!()
                }
                NodeLibrarySingleDefinition::TypeDefinition(name, option_clause) => {
                    let result = name.visit(emitter);

                    match result {
                        Err(msg) => Err(msg),
                        _ => match option_clause {
                            Some(clauses) => {
                                for clause in clauses {
                                    clause.visit(emitter)?;
                                }
                                Ok(TraversalResult::Continue)
                            }
                            None => Ok(TraversalResult::Continue),
                        },
                    }
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_library_single_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeContractDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_contract_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.parameters.visit(emitter)?;

            if let Some(constraint) = &self.constraint {
                constraint.visit(emitter)?;
            }

            for field in &self.fields {
                field.visit(emitter)?;
            }

            for component in &self.components {
                component.visit(emitter)?;
            }

            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_contract_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeContractField {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_contract_field(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self.typed_identifier.visit(emitter) {
                Err(msg) => Err(msg),
                _ => self.right_hand_side.visit(emitter),
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => emitter.emit_contract_field(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeWithConstraint {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_with_constraint(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.expression.visit(emitter)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_with_constraint(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeComponentDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_component_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeComponentDefinition::TransitionComponent(transition_definition) => {
                    transition_definition.visit(emitter)
                }
                NodeComponentDefinition::ProcedureComponent(procedure_definition) => {
                    procedure_definition.visit(emitter)
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_component_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeProcedureDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_procedure_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            let result = self.name.visit(emitter);
            match result {
                Err(msg) => Err(msg),
                _ => match self.parameters.visit(emitter) {
                    Err(msg) => Err(msg),
                    _ => self.body.visit(emitter),
                },
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_procedure_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl AstVisitor for NodeTransitionDefinition {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_transition_definition(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.name.visit(emitter)?;
            self.parameters.visit(emitter)?;
            self.body.visit(emitter)
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_transition_definition(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeAlternativeClause {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        match emitter.emit_type_alternative_clause(TreeTraversalMode::Enter, self)? {
            TraversalResult::SkipChildren => return Ok(TraversalResult::Continue),
            TraversalResult::Continue => (),
        }
        let children_ret = match self {
            NodeTypeAlternativeClause::ClauseType(type_name) => type_name.visit(emitter),
            NodeTypeAlternativeClause::ClauseTypeWithArgs(type_name, type_args) => {
                type_name.visit(emitter)?;

                for type_arg in type_args {
                    type_arg.visit(emitter)?;
                }

                Ok(TraversalResult::Continue)
            }
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_type_alternative_clause(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapValueArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret = emitter.emit_type_map_value_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapValueArguments::EnclosedTypeMapValue(enclosed) => {
                    enclosed.visit(emitter)
                }
                NodeTypeMapValueArguments::GenericMapValueArgument(meta_identifier) => {
                    meta_identifier.visit(emitter)
                }
                NodeTypeMapValueArguments::MapKeyValueType(key_type, value_type) => {
                    match key_type.visit(emitter) {
                        Err(msg) => Err(msg),
                        _ => value_type.visit(emitter),
                    }
                }
            }
        } else {
            ret
        };
        match children_ret? {
            TraversalResult::Continue => {
                emitter.emit_type_map_value_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl AstVisitor for NodeTypeMapValueAllowingTypeArguments {
    fn visit(&self, emitter: &mut dyn AstConverting) -> Result<TraversalResult, String> {
        let ret =
            emitter.emit_type_map_value_allowing_type_arguments(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                NodeTypeMapValueAllowingTypeArguments::TypeMapValueNoArgs(type_map_value) => {
                    type_map_value.visit(emitter)
                }
                NodeTypeMapValueAllowingTypeArguments::TypeMapValueWithArgs(
                    meta_id,
                    value_args,
                ) => {
                    let mut ret = meta_id.visit(emitter);

                    if ret == Ok(TraversalResult::Continue) {
                        for value_arg in value_args {
                            ret = value_arg.visit(emitter);
                            if ret != Ok(TraversalResult::Continue) {
                                break;
                            }
                        }
                    }

                    ret
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.emit_type_map_value_allowing_type_arguments(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
