use std::{path::Path, str::FromStr};

use crate::{
    ast::{
        converting::AstConverting, nodes::NodeContractDefinition, visitor::AstVisitor,
        TraversalResult,
    },
    parser::{lexer::Lexer, parser},
    run_scilla_fmt, Error, FieldList, TransitionList,
};

#[derive(Debug, PartialEq, Default)]
/// The `Contract` struct represents a parsed contract in Rust, including its name, initialization
/// parameters, fields, and transitions.
pub struct Contract {
    /// Name of the parsed contract
    pub name: String,
    /// List of parameters needed to deploy the contract.
    pub init_params: FieldList,
    /// List of the contract's fields.
    pub fields: FieldList,
    /// List of the contract's transitions.
    pub transitions: TransitionList,
}

impl FromStr for Contract {
    type Err = Error;

    /// Parse a Contract from a string slice
    /// # Example
    /// ```
    /// use std::{error::Error, path::PathBuf};
    /// use scilla_parser::{run_scilla_fmt, Contract, Field, FieldList, Transition, TransitionList, Type};
    /// let contract = run_scilla_fmt(&PathBuf::from("tests/contracts/chainid.scilla")).unwrap();
    ///
    /// let contract = contract.parse::<Contract>().unwrap();
    /// assert_eq!(
    ///     contract,
    ///     Contract {
    ///         name: "ChainId".to_string(),
    ///         fields: FieldList::default(),
    ///         init_params: FieldList::default(),
    ///         transitions: TransitionList(vec![Transition::new(
    ///             "EventChainID",
    ///             FieldList::default()
    ///         )])
    ///     }
    /// );
    /// ```
    fn from_str(contract: &str) -> Result<Self, Self::Err> {
        let mut errors = vec![];
        let parsed = parser::ProgramParser::new().parse(&mut errors, Lexer::new(&contract))?;

        let mut contract = Self::default();
        parsed
            .visit(&mut contract)
            .map_err(|e| Error::AstVisitError(e))?;
        Ok(contract)
        // println!("{contract:?}");
        // // todo!()
        // // Bug in lexpr crate requires escaping backslashes
        // let v = lexpr::from_str(&sexp.replace("\\", ""))?;
        // let name = v["contr"][0]["cname"]["Ident"][0][1].to_string();
        // let transitions = (&v["contr"][0]["ccomps"][0]).try_into()?;
        // let init_params = (&v["contr"][0]["cparams"][0]).try_into()?;
        // let fields = (&v["contr"][0]["cfields"][0]).try_into()?;
        // Ok(Contract {
        //     name,
        //     transitions,
        //     init_params,
        //     fields,
        // })
    }
}

impl Contract {
    /// Parse a contract from a given path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{error::Error, path::PathBuf};
    /// use scilla_parser::{Contract, Field, FieldList, Transition, TransitionList, Type};
    /// let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    /// let contract = Contract::from_path(&contract_path).unwrap();
    /// assert_eq!(
    ///     contract,
    ///     Contract {
    ///         name: "ChainId".to_string(),
    ///         fields: FieldList::default(),
    ///         init_params: FieldList::default(),
    ///         transitions: TransitionList(vec![Transition::new(
    ///             "EventChainID",
    ///             FieldList::default()
    ///         )])
    ///     }
    /// );
    /// ```
    pub fn parse(contract_path: &Path) -> Result<Self, Error> {
        // run_scilla_fmt(contract_path)?.parse();
        std::fs::read_to_string(contract_path)?.parse()
    }
}

impl AstConverting for Contract {
    fn push_source_position(
        &mut self,
        start: &crate::parser::lexer::SourcePosition,
        end: &crate::parser::lexer::SourcePosition,
    ) -> () {
    }

    fn pop_source_position(&mut self) -> () {}

    fn emit_byte_str(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeByteStr,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_type_name_identifier(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeNameIdentifier,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_imported_name(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeImportedName,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_import_declarations(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeImportDeclarations,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_meta_identifier(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeMetaIdentifier,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_variable_identifier(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeVariableIdentifier,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_builtin_arguments(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeBuiltinArguments,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_type_map_key(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeMapKey,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_type_map_value(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeMapValue,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_type_argument(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeArgument,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_scilla_type(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeScillaType,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_type_map_entry(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeMapEntry,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_address_type_field(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeAddressTypeField,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_address_type(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeAddressType,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_full_expression(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeFullExpression,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_message_entry(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeMessageEntry,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_pattern_match_expression_clause(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodePatternMatchExpressionClause,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_atomic_expression(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeAtomicExpression,
    ) -> Result<crate::ast::TraversalResult, String> {
        unimplemented!();
    }

    fn emit_contract_type_arguments(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeContractTypeArguments,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_value_literal(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeValueLiteral,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_map_access(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeMapAccess,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_pattern(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodePattern,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_argument_pattern(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeArgumentPattern,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_pattern_match_clause(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodePatternMatchClause,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_blockchain_fetch_arguments(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeBlockchainFetchArguments,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_statement(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeStatement,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_remote_fetch_statement(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeRemoteFetchStatement,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_component_id(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeComponentId,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_component_parameters(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeComponentParameters,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_parameter_pair(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeParameterPair,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_component_body(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeComponentBody,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_statement_block(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeStatementBlock,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_typed_identifier(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypedIdentifier,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_type_annotation(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeAnnotation,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_program(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeProgram,
    ) -> Result<crate::ast::TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn emit_library_definition(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeLibraryDefinition,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_library_single_definition(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeLibrarySingleDefinition,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_contract_definition(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &NodeContractDefinition,
    ) -> Result<crate::ast::TraversalResult, String> {
        self.name = node.contract_name.to_string();
        Ok(TraversalResult::SkipChildren)
    }

    fn emit_contract_field(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeContractField,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_with_constraint(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeWithConstraint,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_component_definition(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeComponentDefinition,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_procedure_definition(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeProcedureDefinition,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_transition_definition(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTransitionDefinition,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_type_alternative_clause(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeAlternativeClause,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_type_map_value_arguments(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeMapValueArguments,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }

    fn emit_type_map_value_allowing_type_arguments(
        &mut self,
        mode: crate::ast::TreeTraversalMode,
        node: &crate::ast::nodes::NodeTypeMapValueAllowingTypeArguments,
    ) -> Result<crate::ast::TraversalResult, String> {
        todo!()
    }
}
