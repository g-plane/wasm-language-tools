use insta::assert_json_snapshot;
use lspt::{DefinitionParams, Position, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, position: Position) -> DefinitionParams {
    DefinitionParams {
        text_document: TextDocumentIdentifier { uri },
        position,
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (cal 0) ;; typo
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 1,
                character: 4
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 29
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 7
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 25
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 14
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 23
            }
        ))
        .is_none());
}

#[test]
fn func_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func
        (call 1) (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 15
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 25
            }
        ))
        .is_none());
}

#[test]
fn func_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func
        (call 0)
    )
)
(module (func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 15,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn func_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func
        (call $func)
    )
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_func_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (func))
    (func
        (call 0)
    )
)
(module (import \"\" \"\" (func)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 15,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_func_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (func $func))
    (func
        (call $func)
    )
)
(module (import \"\" \"\" (func $func)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn func_in_implicit_module() {
    let uri = "untitled:test".to_string();
    let source = "
(func $func)
(func (call $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 2,
            character: 16,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn param_or_local_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i64) (local $local i64))
    (func
        (local.get 0) (local.get $param) (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 20
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 37
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri,
            Position {
                line: 4,
                character: 57
            }
        ))
        .is_none());
}

#[test]
fn param_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param v128)
        (local.get 0)
    )
    (func (param v128))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 20,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn param_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param v128)
        (local.get $param)
    )
    (func (param $param v128))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 25,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn param_in_implicit_module() {
    let uri = "untitled:test".to_string();
    let source = "
(func $func (param i32)
    (local.get 0)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 2,
            character: 16,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn local_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local v128)
        (local.get 0)
    )
    (func (local v128))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 20,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn local_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local v128)
        (local.get $local)
    )
    (func (local $local v128))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 25,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (type 0))
    (func (type $type))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 17
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 18
            }
        ))
        .is_none());
}

#[test]
fn type_use_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (func))
    (func (type 0))
)
(module (type (func)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func))
    (func (type $type))
)
(module (type $type (func)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn global_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func
        (global.get 0) (global.get $global)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 21
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 40
            }
        ))
        .is_none());
}

#[test]
fn global_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32)
    (func
        (global.get 0)
    )
)
(module (global i32))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 21,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn global_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global i32)
    (func
        (global.get $global)
    )
)
(module (global $global i32))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 26,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_global_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (global i32))
    (func
        (global.get 0)
    )
)
(module (import \"\" \"\" (global i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 21,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_global_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (global $global i32))
    (func
        (global.get $global)
    )
)
(module (import \"\" \"\" (global $global i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 26,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn exported_global_int_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (global i32)
    (export "" (global 0))
)
(module (global i32))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 23,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn exported_global_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (global $global i32)
    (export "" (global $global))
)
(module (global $global i32))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 26,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn memory_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export \"\" (memory $memory))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 27
            }
        ))
        .is_none());
}

#[test]
fn memory_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (memory (data))
    (export \"\" (memory 0))
)
(module (memory))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 24,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn memory_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (memory $memory (data))
    (export \"\" (memory $memory))
)
(module (memory $memory))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 30,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_memory_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (memory))
    (export \"\" (memory 0))
)
(module (import \"\" \"\" (memory)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 24,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_memory_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (memory $memory))
    (export \"\" (memory $memory))
)
(module (import \"\" \"\" (memory $memory)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 3,
            character: 30,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn table_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (table.size $table)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 21
            }
        ))
        .is_none());
}

#[test]
fn table_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func
    (table.size 0)))
(module
  (table))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn table_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table)))
(module
  (table $table))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 22,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_table_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (import \"\" \"\" (table 0 funcref))
  (func
    (table.size 0)))
(module
  (import \"\" \"\" (table 0 funcref)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn imported_table_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (import \"\" \"\" (table $table 0 funcref))
  (func
    (table.size $table)))
(module
  (import \"\" \"\" (table $table 0 funcref)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 22,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn block_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (br_table 0 $block))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 20
            }
        ))
        .is_none());
    assert!(service
        .goto_definition(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 27
            }
        ))
        .is_none());
}

#[test]
fn block_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block
      (br_table 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 16,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn block_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $block
      (br_table $block))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_definition(create_params(
        uri,
        Position {
            line: 4,
            character: 21,
        },
    ));
    assert_json_snapshot!(response);
}
