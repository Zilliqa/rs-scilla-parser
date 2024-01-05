use std::fmt::Display;

use crate::simplified_representation::primitives::SrType;

/// Represents all different scilla types.
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int32,
    Int64,
    Int128,
    Int256,

    Uint32,
    Uint64,
    Uint128,
    Uint256,

    String,

    BNum,
    Map(Box<Type>, Box<Type>),

    ByStr(usize),

    // ADT
    Bool,
    Option(Box<Type>),
    Pair(Box<Type>, Box<Type>),
    List(Box<Type>),

    Other(String),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int32 => write!(f, "Int32"),
            Type::Int64 => write!(f, "Int64"),
            Type::Int128 => write!(f, "Int128"),
            Type::Int256 => write!(f, "Int256"),
            Type::Uint32 => write!(f, "Uint32"),
            Type::Uint64 => write!(f, "Uint64"),
            Type::Uint128 => write!(f, "Uint128"),
            Type::Uint256 => write!(f, "Uint256"),
            Type::String => write!(f, "String"),
            Type::BNum => write!(f, "BNum"),
            Type::Bool => write!(f, "Bool"),
            Type::Map(ref k, ref v) => write!(f, "(Map {}, {})", k, v),
            Type::Option(ref k) => write!(f, "(Option {})", k),
            Type::List(ref k) => write!(f, "(List {})", k),
            Type::Pair(ref k, ref v) => write!(f, "(Pair {} {})", k, v),
            Type::ByStr(n) => write!(f, "ByStr{}", n),
            Type::Other(ref s) => write!(f, "{}", s),
        }
    }
}

impl From<SrType> for Type {
    fn from(mut type_definition: SrType) -> Self {
        match type_definition.main_type.as_str() {
            "Int32" => Type::Int32,
            "Int64" => Type::Int64,
            "Int128" => Type::Int128,
            "Int256" => Type::Int256,
            "Uint32" => Type::Uint32,
            "Uint64" => Type::Uint64,
            "Uint128" => Type::Uint128,
            "Uint256" => Type::Uint256,
            "String" => Type::String,
            "ByStr20" => Type::ByStr(20),
            "BNum" => Type::BNum,
            "Bool" => Type::Bool,
            // TODO: Remove unwrap
            "Option" => Type::Option(Box::new(type_definition.sub_types.pop().unwrap().into())),
            "List" => Type::List(Box::new(type_definition.sub_types.pop().unwrap().into())),
            "Pair" => {
                let t2 = type_definition.sub_types.pop().unwrap();
                let t1 = type_definition.sub_types.pop().unwrap();
                Type::Pair(Box::new(t1.into()), Box::new(t2.into()))
            }
            "Map" => {
                let value = type_definition.sub_types.pop().unwrap();
                let key = type_definition.sub_types.pop().unwrap();
                Type::Map(Box::new(key.into()), Box::new(value.into()))
            }
            _ => todo!("{type_definition:#?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_to_string() {
        //(List (Pair ByStr20 (List (Pair ByStr20 Uint32))))
        let list_type = Type::List(Box::new(Type::Pair(
            Box::new(Type::ByStr(20)),
            Box::new(Type::List(Box::new(Type::Pair(
                Box::new(Type::ByStr(20)),
                Box::new(Type::Uint32),
            )))),
        )));

        assert_eq!(
            "(List (Pair ByStr20 (List (Pair ByStr20 Uint32))))",
            list_type.to_string()
        );

        // (List (Pair ByStr20 (List (Pair ByStr20 (List (Pair Uint32 Uint128))))))
        let list_type = Type::List(Box::new(Type::Pair(
            Box::new(Type::ByStr(20)),
            Box::new(Type::List(Box::new(Type::Pair(
                Box::new(Type::ByStr(20)),
                Box::new(Type::List(Box::new(Type::Pair(
                    Box::new(Type::Uint32),
                    Box::new(Type::Uint128),
                )))),
            )))),
        )));

        assert_eq!(
            "(List (Pair ByStr20 (List (Pair ByStr20 (List (Pair Uint32 Uint128))))))",
            list_type.to_string()
        );
    }
}
