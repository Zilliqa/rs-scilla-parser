use pretty_assertions::assert_eq;
use std::{error::Error, path::PathBuf};

use scilla_parser::{Contract, Field, FieldList, Transition, TransitionList, Type};

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    for entry in std::fs::read_dir("tests/contracts")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("Parsing {}", path.display());
            Contract::parse(&path)?;
        }
    }
    Ok(())
}

#[test]
fn test_map_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/Map.scilla");
    let contract = Contract::parse(&contract_path).unwrap();
    assert_eq!(
        contract,
        Contract {
            name: "DifferentMaps".to_string(),
            init_params: FieldList::default(),
            fields: FieldList(vec![
                Field::new(
                    "first_map",
                    Type::Map(Box::new(Type::String), Box::new(Type::BNum))
                ),
                Field::new(
                    "status3days",
                    Type::Map(
                        Box::new(Type::String),
                        Box::new(Type::Pair(Box::new(Type::ByStr20), Box::new(Type::BNum)))
                    )
                ),
                Field::new(
                    "reward_pairs",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::List(Box::new(Type::Uint128)))
                    )
                )
            ]),
            transitions: TransitionList::default()
        }
    );
}

#[test]
fn test_bystr_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/ByStr.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "AllByStrVariants".to_string(),
            init_params: FieldList(vec![
                Field::new("bystr", Type::ByStr),
                Field::new("bystr32", Type::ByStrX(32)),
                Field::new("raw_address", Type::ByStr20),
                Field::new(
                    "library_address",
                    Type::ByStr20With {
                        type_name: "library".to_string(),
                        fields: FieldList::default()
                    }
                ),
                Field::new(
                    "contract_address",
                    Type::ByStr20With {
                        type_name: "contract".to_string(),
                        fields: FieldList::default()
                    }
                ),
                Field::new(
                    "detailed_contract_address",
                    Type::ByStr20With {
                        type_name: "contract".to_string(),
                        fields: FieldList(vec![
                            Field::new(
                                "allowances",
                                Type::Map(
                                    Box::new(Type::ByStr20),
                                    Box::new(Type::Map(
                                        Box::new(Type::ByStr20),
                                        Box::new(Type::Uint128)
                                    ))
                                )
                            ),
                            Field::new(
                                "balances",
                                Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128))
                            ),
                            Field::new("total_supply", Type::Uint128)
                        ])
                    }
                ),
                Field::new(
                    "complex_contract_address",
                    Type::ByStr20With {
                        type_name: "contract".to_string(),
                        fields: FieldList(vec![
                            Field::new(
                                "implementation",
                                Type::ByStr20With {
                                    type_name: "contract".to_string(),
                                    fields: FieldList(vec![
                                        Field::new(
                                            "services",
                                            Type::Map(
                                                Box::new(Type::String),
                                                Box::new(Type::ByStr20)
                                            )
                                        ),
                                        Field::new(
                                            "utility",
                                            Type::Map(
                                                Box::new(Type::String),
                                                Box::new(Type::Uint128)
                                            )
                                        ),
                                    ])
                                }
                            ),
                            Field::new(
                                "dns",
                                Type::Map(Box::new(Type::String), Box::new(Type::ByStr20))
                            ),
                            Field::new(
                                "guardians",
                                Type::Map(
                                    Box::new(Type::String),
                                    Box::new(Type::ByStr20With {
                                        type_name: "contract".to_string(),
                                        fields: FieldList(vec![Field::new(
                                            "verification_methods",
                                            Type::Map(
                                                Box::new(Type::String),
                                                Box::new(Type::ByStrX(33))
                                            )
                                        ),])
                                    }),
                                )
                            )
                        ])
                    }
                )
            ]),
            fields: FieldList::default(),
            transitions: TransitionList(vec![
                Transition::new(
                    "ArbitrageFromXCAD",
                    FieldList(vec![Field::new(
                        "token",
                        Type::ByStr20With {
                            type_name: "contract".to_string(),
                            fields: FieldList(vec![Field::new(
                                "balances",
                                Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128))
                            ),],),
                        },
                    )]),
                ),
                Transition::new(
                    "BuyNFTUsername",
                    FieldList(vec![
                        Field::new("username", Type::String),
                        Field::new(
                            "guardianship",
                            Type::Option(Box::new(Type::ByStr20With {
                                type_name: "contract".to_string(),
                                fields: FieldList(vec![Field::new(
                                    "verification_methods",
                                    Type::Map(Box::new(Type::String), Box::new(Type::ByStrX(33)))
                                )])
                            }))
                        ),
                        Field::new("id", Type::String),
                        Field::new("tyron", Type::Option(Box::new(Type::Uint128))),
                    ])
                )
            ])
        }
    );
}

#[test]
fn test_timestamp_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/Timestamp.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "Timestamp".to_string(),
            init_params: FieldList::default(),
            fields: FieldList::default(),
            transitions: TransitionList(vec![Transition::new(
                "EventTimestamp",
                FieldList(vec![Field::new("bnum", Type::BNum)])
            )])
        }
    );
}

#[test]
fn test_staking_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/StakingContract.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "StakingContract".to_string(),
            init_params: FieldList(vec![
                Field::new("initial_owner", Type::ByStr20),
                Field::new(
                    "initial_staking_token_address",
                    Type::ByStr20With {
                        type_name: "contract".to_string(),
                        fields: FieldList(vec![
                            Field::new(
                                "allowances",
                                Type::Map(
                                    Box::new(Type::ByStr20),
                                    Box::new(Type::Map(
                                        Box::new(Type::ByStr20),
                                        Box::new(Type::Uint128)
                                    ))
                                )
                            ),
                            Field::new(
                                "balances",
                                Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128))
                            ),
                            Field::new("total_supply", Type::Uint128)
                        ])
                    }
                )
            ]),
            fields: FieldList(vec![
                Field::new("owner", Type::ByStr20),
                Field::new("staking_token_address", Type::ByStr20),
                Field::new("pending_owner", Type::ByStr20),
                Field::new("paused", Type::Bool),
                Field::new(
                    "reward_pairs",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::List(Box::new(Type::Other("RewardParam".to_string()))))
                    )
                ),
                Field::new(
                    "stakes",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::Other("Stake".to_string()))
                    )
                ),
                Field::new(
                    "rewards",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128)))
                    )
                ),
                Field::new(
                    "administrators",
                    Type::Map(Box::new(Type::ByStr20), Box::new(Type::Bool))
                ),
                Field::new(
                    "treasury_balances",
                    Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128))
                ),
                Field::new("treasury_fees_address", Type::ByStr20),
                Field::new("penalty_fee_balances", Type::Uint128),
                Field::new("total_staked_amount", Type::Uint128),
            ]),
            transitions: TransitionList(vec![
                Transition::new(
                    "AddStake",
                    FieldList(vec![
                        Field::new("amount", Type::Uint128),
                        Field::new("expiration_time", Type::Uint64),
                        Field::new("penalty_fee_bps", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "RemoveStake",
                    FieldList(vec![Field::new("amount", Type::Uint128,),],)
                ),
                Transition::new("ClaimRewards", FieldList::default()),
                Transition::new(
                    "AddRewardToken",
                    FieldList(vec![
                        Field::new("reward_token_address", Type::ByStr20),
                        Field::new("apr", Type::Uint128,),
                        Field::new("treasury_fee", Type::Uint128)
                    ]),
                ),
                Transition::new(
                    "RemoveRewardToken",
                    FieldList(vec![Field::new("reward_token_address", Type::ByStr20)]),
                ),
                Transition::new("RemoveAllRewardTokens", FieldList::default()),
                Transition::new("Pause", FieldList::default()),
                Transition::new("UnPause", FieldList::default()),
                Transition::new(
                    "AddAdmin",
                    FieldList(vec![Field::new("address", Type::ByStr20)])
                ),
                Transition::new(
                    "RemoveAdmin",
                    FieldList(vec![Field::new("address", Type::ByStr20)])
                ),
                Transition::new(
                    "TransferOwnership",
                    FieldList(vec![Field::new("new_owner", Type::ByStr20)])
                ),
                Transition::new("AcceptPendingOwnership", FieldList::default()),
                Transition::new(
                    "WithdrawTokens",
                    FieldList(vec![
                        Field::new("token_address", Type::ByStr20),
                        Field::new("token_amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "WithdrawZils",
                    FieldList(vec![Field::new("zil_amount", Type::Uint128),])
                ),
                Transition::new(
                    "Deposit",
                    FieldList(vec![
                        Field::new("token_address", Type::ByStr20),
                        Field::new("token_amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "TransferFromSuccessCallBack",
                    FieldList(vec![
                        Field::new("initiator", Type::ByStr20),
                        Field::new("sender", Type::ByStr20),
                        Field::new("recipient", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "TransferSuccessCallBack",
                    FieldList(vec![
                        Field::new("sender", Type::ByStr20),
                        Field::new("recipient", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "RecipientAcceptTransferFrom",
                    FieldList(vec![
                        Field::new("initiator", Type::ByStr20),
                        Field::new("sender", Type::ByStr20),
                        Field::new("recipient", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new("AddFunds", FieldList::default()),
                Transition::new(
                    "RecipientAcceptTransfer",
                    FieldList(vec![
                        Field::new("sender", Type::ByStr20),
                        Field::new("recipient", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
            ])
        }
    );
}

#[test]
fn test_chain_id_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/chainid.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "ChainId".to_string(),
            fields: FieldList(vec![Field::new("dummy_field", Type::Uint256)]),
            init_params: FieldList::default(),
            transitions: TransitionList(vec![Transition::new(
                "EventChainID",
                FieldList::default()
            )])
        }
    );
}

#[test]
fn test_hello_world_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/HelloWorld.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "HelloWorld".to_string(),
            init_params: FieldList(vec![Field::new("owner", Type::ByStr20)]),
            fields: FieldList(vec![Field::new("welcome_msg", Type::String)]),
            transitions: TransitionList(vec![
                Transition::new("setHello", FieldList(vec![Field::new("msg", Type::String)])),
                Transition::new_without_param("getHello")
            ])
        }
    );
}

#[test]
fn test_get_fields_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/GetFields.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "GetFields".to_string(),
            init_params: FieldList::default(),
            fields: FieldList(vec![
                Field::new("field_uint32", Type::Uint32),
                Field::new("field_uint64", Type::Uint64),
                Field::new("field_uint128", Type::Uint128),
                Field::new("field_uint256", Type::Uint256),
                Field::new("field_int32", Type::Int32),
                Field::new("field_int64", Type::Int64),
                Field::new("field_int128", Type::Int128),
                Field::new("field_bnum", Type::BNum),
                Field::new("field_string", Type::String),
                Field::new("field_address", Type::ByStr20),
                Field::new("field_bool_false", Type::Bool),
                Field::new("field_bool_true", Type::Bool),
                Field::new(
                    "field_option_bystr20_none",
                    Type::Option(Box::new(Type::ByStr20))
                ),
                Field::new(
                    "field_option_bystr20_some",
                    Type::Option(Box::new(Type::ByStr20))
                ),
                Field::new(
                    "field_option_int32_some",
                    Type::Option(Box::new(Type::Int32))
                ),
                Field::new("field_option_bool_some", Type::Option(Box::new(Type::Bool))),
                Field::new(
                    "field_pair",
                    Type::Pair(Box::new(Type::String), Box::new(Type::Uint32))
                ),
                Field::new(
                    "balances",
                    Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128))
                ),
                Field::new("field_list", Type::List(Box::new(Type::Int32)))
            ]),
            transitions: TransitionList::default(),
        }
    );
}

#[test]
fn test_call_transition_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/CallTransition.scilla");
    let contract = Contract::parse(&contract_path).unwrap();
    assert_eq!(
        contract,
        Contract {
            name: "CallTransition".to_string(),
            init_params: FieldList::default(),
            fields: FieldList::default(),
            transitions: TransitionList(vec![
                Transition::new(
                    "call_uint32",
                    FieldList(vec![Field::new("v", Type::Uint32)])
                ),
                Transition::new(
                    "call_uint64",
                    FieldList(vec![Field::new("v", Type::Uint64)])
                ),
                Transition::new(
                    "call_uint128",
                    FieldList(vec![Field::new("v", Type::Uint128)])
                ),
                Transition::new(
                    "call_uint256",
                    FieldList(vec![Field::new("v", Type::Uint256)])
                ),
                Transition::new("call_int32", FieldList(vec![Field::new("v", Type::Int32)])),
                Transition::new("call_int64", FieldList(vec![Field::new("v", Type::Int64)])),
                Transition::new(
                    "call_int128",
                    FieldList(vec![Field::new("v", Type::Int128)])
                ),
                Transition::new(
                    "call_string",
                    FieldList(vec![Field::new("v", Type::String)])
                ),
                Transition::new(
                    "call_address",
                    FieldList(vec![Field::new("v", Type::ByStr20)])
                ),
                Transition::new(
                    "call_option_bool",
                    FieldList(vec![Field::new("v", Type::Option(Box::new(Type::Bool)))])
                ),
                Transition::new("call_bool", FieldList(vec![Field::new("v", Type::Bool)])),
                Transition::new("call_bnum", FieldList(vec![Field::new("v", Type::BNum)])),
                Transition::new(
                    "call_pair",
                    FieldList(vec![Field::new(
                        "v",
                        Type::Pair(Box::new(Type::String), Box::new(Type::Uint32))
                    )])
                ),
                Transition::new(
                    "call_list",
                    FieldList(vec![Field::new("v", Type::List(Box::new(Type::ByStr20)))])
                ),
                Transition::new(
                    "call_list_2",
                    FieldList(vec![Field::new(
                        "v",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::Uint32)
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "call_list_3",
                    FieldList(vec![Field::new(
                        "v",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::String),
                            Box::new(Type::String)
                        )))
                    )])
                ),
                Transition::new(
                    "call_list_4",
                    FieldList(vec![Field::new(
                        "v",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                ))))
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "call_list_5",
                    FieldList(vec![Field::new(
                        "v",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::BNum),
                                Box::new(Type::Uint128)
                            ))))
                        )))
                    )])
                ),
            ])
        }
    );
}

#[test]
fn test_send_zil_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/SendZil.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "SendZil".to_string(),
            init_params: FieldList::default(),
            fields: FieldList(vec![
                Field::new("test_field", Type::Uint256),
                Field::new("bool", Type::Bool),
                Field::new("empty_bool", Type::Option(Box::new(Type::Bool))),
                Field::new("some_int", Type::Option(Box::new(Type::Int32))),
                Field::new(
                    "pair",
                    Type::Pair(Box::new(Type::String), Box::new(Type::Uint32))
                ),
                Field::new("list", Type::List(Box::new(Type::Int32))),
            ]),
            transitions: TransitionList(vec![
                Transition::new_without_param("acceptZil"),
                Transition::new(
                    "updateTestField",
                    FieldList(vec![Field::new("val", Type::Uint256)])
                ),
                Transition::new_without_param("dontAcceptZil"),
                Transition::new(
                    "fundUserWithTag",
                    FieldList(vec![
                        Field::new("user", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "fundUser",
                    FieldList(vec![
                        Field::new("user", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "fundContract",
                    FieldList(vec![
                        Field::new("contract_address", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "callOtherContract",
                    FieldList(vec![
                        Field::new("contract_address", Type::ByStr20),
                        Field::new("tag", Type::String),
                        Field::new("value", Type::Uint256)
                    ])
                ),
            ])
        }
    );
}

#[test]
fn test_fungible_token_parse() {
    let contract_path = PathBuf::from("tests/contracts/FungibleToken.scilla");
    let contract = Contract::parse(&contract_path).unwrap();
    assert_eq!(
        contract,
        Contract {
            name: "FungibleToken".to_string(),
            init_params: FieldList(vec![
                Field::new("contract_owner", Type::ByStr20),
                Field::new("name", Type::String),
                Field::new("symbol", Type::String),
                Field::new("decimals", Type::Uint32),
                Field::new("init_supply", Type::Uint128)
            ]),
            fields: FieldList(vec![
                Field::new("total_supply", Type::Uint128),
                Field::new(
                    "balances",
                    Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128))
                ),
                Field::new(
                    "allowances",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128)))
                    )
                )
            ]),
            transitions: TransitionList(vec![
                Transition::new(
                    "IncreaseAllowance",
                    FieldList(vec![
                        Field::new("spender", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "DecreaseAllowance",
                    FieldList(vec![
                        Field::new("spender", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "Transfer",
                    FieldList(vec![
                        Field::new("to", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "TransferFailed",
                    FieldList(vec![
                        Field::new("to", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
                Transition::new(
                    "TransferFrom",
                    FieldList(vec![
                        Field::new("from", Type::ByStr20),
                        Field::new("to", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ])
                ),
            ])
        }
    );
}

#[test]
fn test_staking_proxy_v2_parse() {
    let contract_path = PathBuf::from("tests/contracts/staking_proxy_v2.scilla");
    let contract = Contract::parse(&contract_path).unwrap();
    assert_eq!(
        contract,
        Contract {
            name: "SSNListProxy_V2".to_string(),
            init_params: FieldList(vec![
                Field::new("init_implementation", Type::ByStr20),
                Field::new("init_admin", Type::ByStr20),
            ]),
            fields: FieldList(vec![
                Field::new("implementation", Type::ByStr20),
                Field::new("admin", Type::ByStr20),
                Field::new("stagingadmin", Type::Option(Box::new(Type::ByStr20))),
            ]),
            transitions: TransitionList(vec![
                Transition::new(
                    "UpgradeTo",
                    FieldList(vec![Field::new("newImplementation", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeProxyAdmin",
                    FieldList(vec![Field::new("newAdmin", Type::ByStr20)])
                ),
                Transition::new_without_param("ClaimProxyAdmin"),
                Transition::new(
                    "OptInSSNToConsensusPoolAdminOverride",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr20)])
                ),
                Transition::new(
                    "OptOutSSNFromConsensusPoolAdminOverride",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr20)])
                ),
                Transition::new(
                    "RemoveFromConsensusPoolAdminOverride",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeMinCommissionRate",
                    FieldList(vec![Field::new("mincommrate_value", Type::Uint128)])
                ),
                Transition::new(
                    "AddSSNNonStaking",
                    FieldList(vec![
                        Field::new("ssnaddr", Type::ByStr20),
                        Field::new("name", Type::String),
                        Field::new("urlraw", Type::String),
                        Field::new("urlapi", Type::String),
                        Field::new("comm", Type::Uint128)
                    ])
                ),
                Transition::new_without_param("AddSSNToConsensusPool"),
                Transition::new_without_param("RemoveSSNFromConsensusPool"),
                Transition::new(
                    "WithdrawStakeRewardsForCycles",
                    FieldList(vec![
                        Field::new("ssnaddr", Type::ByStr20),
                        Field::new("cycles", Type::Uint32)
                    ])
                ),
                Transition::new(
                    "CopySSNDelegAmt",
                    FieldList(vec![
                        Field::new("ssn", Type::ByStr20),
                        Field::new(
                            "keys",
                            Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::Uint128)
                            )))
                        )
                    ])
                ),
                Transition::new(
                    "MigrateStakeSSNPerCycle",
                    FieldList(vec![
                        Field::new("ssn", Type::ByStr20),
                        Field::new(
                            "keys",
                            Type::List(Box::new(Type::Pair(
                                Box::new(Type::Uint32),
                                Box::new(Type::Pair(
                                    Box::new(Type::Uint128),
                                    Box::new(Type::Uint128)
                                ))
                            )))
                        )
                    ])
                ),
                Transition::new(
                    "CopyBuffDepositDeleg",
                    FieldList(vec![
                        Field::new("deleg", Type::ByStr20),
                        Field::new(
                            "keys",
                            Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                ))))
                            )))
                        )
                    ])
                ),
                Transition::new(
                    "CopyLastBufDepositCycleDelegList",
                    FieldList(vec![Field::new(
                        "last_buf_deposit_cycle_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::Uint32),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyLastWithdrawCycleDelegList",
                    FieldList(vec![Field::new(
                        "last_withdraw_cycle_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::Uint32),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDelegStakePerCycleList",
                    FieldList(vec![Field::new(
                        "deleg_stake_per_cycle_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                )))),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDirectDepositDelegList",
                    FieldList(vec![Field::new(
                        "direct_deposit_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                )))),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyBuffDepositDelegList",
                    FieldList(vec![Field::new(
                        "buff_deposit_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::List(Box::new(Type::Pair(
                                    Box::new(Type::Uint32),
                                    Box::new(Type::Uint128)
                                )))),
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDepositAmtDelegList",
                    FieldList(vec![Field::new(
                        "deposit_amt_deleg_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::ByStr20),
                                Box::new(Type::Uint128)
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyWithDrawalPendingList",
                    FieldList(vec![Field::new(
                        "withdrawal_pending_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::BNum),
                                Box::new(Type::Uint128)
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyCommForSSNList",
                    FieldList(vec![Field::new(
                        "comm_for_ssn_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::List(Box::new(Type::Pair(
                                Box::new(Type::Uint32),
                                Box::new(Type::Uint128)
                            ))))
                        )))
                    )])
                ),
                Transition::new(
                    "CopyDelegSwapRequest",
                    FieldList(vec![Field::new(
                        "deleg_swap_request_list",
                        Type::List(Box::new(Type::Pair(
                            Box::new(Type::ByStr20),
                            Box::new(Type::ByStr20),
                        )))
                    )])
                ),
                Transition::new(
                    "ChangeCycleRewardsDeleg",
                    FieldList(vec![Field::new("input_cycle_rewards_deleg", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeVerifierReward",
                    FieldList(vec![Field::new("input_verifier_reward", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeAvailableWithdrawal",
                    FieldList(vec![Field::new(
                        "input_available_withdrawal",
                        Type::Uint128
                    )])
                ),
                Transition::new(
                    "ChangeCurrentDeleg",
                    FieldList(vec![Field::new("input_current_deleg", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeCurrentSSN",
                    FieldList(vec![Field::new("input_current_ssn", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeNewDeleg",
                    FieldList(vec![Field::new("input_new_deleg", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeVerifier",
                    FieldList(vec![Field::new("input_verifier", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeVerifierReceivingAddr",
                    FieldList(vec![Field::new(
                        "input_verifier_receiving_addr",
                        Type::ByStr20
                    )])
                ),
                Transition::new(
                    "ChangeMinStake",
                    FieldList(vec![Field::new("input_minstake", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeMinDelegStake",
                    FieldList(vec![Field::new("input_mindelegstake", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeLastRewardCycle",
                    FieldList(vec![Field::new("input_lastrewardcycle", Type::Uint32)])
                ),
                Transition::new(
                    "ChangeMaxCommChangeRate",
                    FieldList(vec![Field::new("input_maxcommchangerate", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeMaxCommRate",
                    FieldList(vec![Field::new("input_maxcommrate", Type::Uint128)])
                ),
                Transition::new(
                    "ChangeTotalStakeAmount",
                    FieldList(vec![Field::new("input_totalstakeamount", Type::Uint128)])
                ),
            ])
        }
    );
}

#[test]
fn test_stzil_contract_parse() {
    let contract_path = PathBuf::from("tests/contracts/stzil.scilla");
    let contract = Contract::parse(&contract_path).unwrap();

    assert_eq!(
        contract,
        Contract {
            name: "StZIL".to_string(),
            init_params: FieldList(vec![
                Field::new("contract_owner", Type::ByStr20),
                Field::new("init_admin_address", Type::ByStr20),
                Field::new("init_zimpl_address", Type::ByStr20),
                Field::new("name", Type::String),
                Field::new("symbol", Type::String),
                Field::new("decimals", Type::Uint32),
                Field::new("init_supply", Type::Uint128)
            ]),
            fields: FieldList(vec![
                Field::new("owner_address", Type::ByStr20),
                Field::new("admin_address", Type::ByStr20),
                Field::new("treasury_address", Type::ByStr20),
                Field::new("withdrawal_fee_address", Type::ByStr20),
                Field::new("zimpl_address", Type::ByStr20),
                Field::new("holder_address", Type::ByStr20),
                Field::new("buffers_addresses", Type::List(Box::new(Type::ByStr20))),
                Field::new("ssn_addresses", Type::List(Box::new(Type::ByStr20))),
                Field::new(
                    "staging_owner_address",
                    Type::Option(Box::new(Type::ByStr20))
                ),
                Field::new("is_paused_in", Type::Bool),
                Field::new("is_paused_out", Type::Bool),
                Field::new("is_paused_zrc2", Type::Bool),
                Field::new("mindelegstake", Type::Uint128),
                Field::new("withdrawal_fee", Type::Uint128),
                Field::new("rewards_fee", Type::Uint128),
                Field::new("totalstakeamount", Type::Uint128),
                Field::new("autorestakeamount", Type::Uint128),
                Field::new("total_supply", Type::Uint128),
                Field::new(
                    "balances",
                    Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128))
                ),
                Field::new(
                    "allowances",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint128)))
                    )
                ),
                Field::new(
                    "withdrawal_pending",
                    Type::Map(
                        Box::new(Type::BNum),
                        Box::new(Type::Map(
                            Box::new(Type::ByStr20),
                            Box::new(Type::Other("Withdrawal".to_string()))
                        ))
                    )
                ),
                Field::new(
                    "withdrawal_pending_of_delegator",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::Map(
                            Box::new(Type::BNum),
                            Box::new(Type::Other("Withdrawal".to_string()))
                        ))
                    )
                ),
                Field::new(
                    "withdrawal_unbonded",
                    Type::Map(
                        Box::new(Type::ByStr20),
                        Box::new(Type::Other("Withdrawal".to_string()))
                    )
                ),
                Field::new(
                    "buffer_drained_cycle",
                    Type::Map(Box::new(Type::ByStr20), Box::new(Type::Uint32))
                ),
                Field::new("ssn_index", Type::Uint128),
                Field::new("tmp_delegator", Type::Option(Box::new(Type::ByStr20))),
                Field::new("tmp_stake_delegate_amount", Type::Uint128),
                Field::new("tmp_complete_withdrawal_available", Type::Uint128),
                Field::new("tmp_ssn_addr_in", Type::ByStr20),
                Field::new("tmp_ssn_addr_out", Type::ByStr20),
                Field::new("tmp_bnum", Type::BNum),
                Field::new("tmp_deleg_exists", Type::Bool),
                Field::new("local_bnum_req", Type::Uint128),
                Field::new("local_lastrewardcycle", Type::Uint32),
            ],),
            transitions: TransitionList(vec![
                Transition::new_without_param("PauseIn"),
                Transition::new_without_param("UnPauseIn"),
                Transition::new_without_param("PauseOut"),
                Transition::new_without_param("UnPauseOut"),
                Transition::new_without_param("PauseZrc2"),
                Transition::new_without_param("UnPauseZrc2"),
                Transition::new(
                    "ChangeAdmin",
                    FieldList(vec![Field::new("new_admin", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeOwner",
                    FieldList(vec![Field::new("new_owner", Type::ByStr20)])
                ),
                Transition::new_without_param("ClaimOwner"),
                Transition::new(
                    "ChangeTreasuryAddress",
                    FieldList(vec![Field::new("address", Type::ByStr20)])
                ),
                Transition::new(
                    "ChangeWithdrawalFeeAddress",
                    FieldList(vec![Field::new("address", Type::ByStr20)])
                ),
                Transition::new(
                    "SetHolderAddress",
                    FieldList(vec![Field::new("address", Type::ByStr20)]),
                ),
                Transition::new(
                    "ChangeZimplAddress",
                    FieldList(vec![Field::new("address", Type::ByStr20)]),
                ),
                Transition::new(
                    "ChangeBuffers",
                    FieldList(vec![Field::new(
                        "new_buffers",
                        Type::List(Box::new(Type::ByStr20))
                    )])
                ),
                Transition::new(
                    "AddSSN",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr20)])
                ),
                Transition::new(
                    "RemoveSSN",
                    FieldList(vec![Field::new("ssnaddr", Type::ByStr20)])
                ),
                Transition::new(
                    "ClaimRewards",
                    FieldList(vec![
                        Field::new("buffer_or_holder", Type::ByStr20),
                        Field::new("ssn", Type::ByStr20)
                    ])
                ),
                Transition::new(
                    "ConsolidateInHolder",
                    FieldList(vec![Field::new("buffer_addr", Type::ByStr20)])
                ),
                Transition::new_without_param("ClaimRewardsSuccessCallBack"),
                Transition::new_without_param("PerformAutoRestake"),
                Transition::new_without_param("IncreaseAutoRestakeAmount"),
                Transition::new(
                    "UpdateStakingParameters",
                    FieldList(vec![
                        Field::new("new_mindelegstake", Type::Uint128),
                        Field::new("new_rewards_fee", Type::Uint128),
                        Field::new("new_withdrawal_fee", Type::Uint128),
                    ],)
                ),
                Transition::new_without_param("DelegateStake"),
                Transition::new(
                    "DelegateStakeWithReferral",
                    FieldList(vec![Field::new("referral", Type::ByStr20),],)
                ),
                Transition::new(
                    "DelegateStakeSuccessCallBack",
                    FieldList(vec![Field::new("amount", Type::Uint128,),])
                ),
                Transition::new(
                    "ClaimWithdrawal",
                    FieldList(vec![Field::new(
                        "blocks_to_withdraw",
                        Type::List(Box::new(Type::BNum))
                    )],)
                ),
                Transition::new(
                    "WithdrawTokensAmt",
                    FieldList(vec![Field::new("amount", Type::Uint128)],)
                ),
                Transition::new(
                    "SlashSSN",
                    FieldList(vec![
                        Field::new("withdraw_stake_amt", Type::Uint128),
                        Field::new("ssnaddr", Type::ByStr20)
                    ],)
                ),
                Transition::new_without_param("CompleteWithdrawal"),
                Transition::new_without_param("CompleteWithdrawalSuccessCallBack"),
                Transition::new(
                    "ChownStakeConfirmSwap",
                    FieldList(vec![Field::new("delegator", Type::ByStr20),],)
                ),
                Transition::new(
                    "ChownStakeReDelegate",
                    FieldList(vec![
                        Field::new("from_ssn", Type::ByStr20),
                        Field::new("amount", Type::Uint128),
                    ],)
                ),
                Transition::new(
                    "IncreaseAllowance",
                    FieldList(vec![
                        Field::new("spender", Type::ByStr20),
                        Field::new("amount", Type::Uint128),
                    ],)
                ),
                Transition::new(
                    "DecreaseAllowance",
                    FieldList(vec![
                        Field::new("spender", Type::ByStr20),
                        Field::new("amount", Type::Uint128),
                    ],)
                ),
                Transition::new(
                    "Transfer",
                    FieldList(vec![
                        Field::new("to", Type::ByStr20),
                        Field::new("amount", Type::Uint128,),
                    ],)
                ),
                Transition::new(
                    "TransferFrom",
                    FieldList(vec![
                        Field::new("from", Type::ByStr20),
                        Field::new("to", Type::ByStr20),
                        Field::new("amount", Type::Uint128)
                    ],)
                ),
            ])
        }
    )
}
