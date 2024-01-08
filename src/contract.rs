use std::{path::Path, str::FromStr};

use crate::{
    parser::{lexer::Lexer, parser},
    simplified_representation::emitter::SrEmitter,
    Error, FieldList, TransitionList,
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
    /// use scilla_parser::{Contract, Field, FieldList, Transition, TransitionList, Type};
    /// let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    /// let contract_str = include_str!("../tests/contracts/chainid.scilla");
    /// let contract = contract_str.parse::<Contract>().unwrap();
    ///
    /// assert_eq!(
    ///     contract,
    ///     Contract {
    ///         name: "ChainId".to_string(),
    ///         fields: FieldList(vec![Field::new("dummy_field", Type::Uint256)]),
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
        let parsed = parser::ProgramParser::new().parse(&mut errors, Lexer::new(contract))?;

        let emitter = SrEmitter::default();
        emitter.emit(&parsed).map_err(Error::ParseError)
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
    /// let contract = Contract::parse(&contract_path).unwrap();
    ///
    /// assert_eq!(
    ///     contract,
    ///     Contract {
    ///         name: "ChainId".to_string(),
    ///         fields: FieldList(vec![Field::new("dummy_field", Type::Uint256)]),
    ///         init_params: FieldList::default(),
    ///         transitions: TransitionList(vec![Transition::new(
    ///             "EventChainID",
    ///             FieldList::default()
    ///         )])
    ///     }
    /// );
    /// ```
    pub fn parse(contract_path: &Path) -> Result<Self, Error> {
        std::fs::read_to_string(contract_path)?.parse()
    }
}
