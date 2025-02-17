use insta::assert_json_snapshot;
use lspt::{Position, RenameParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, position: Position, new_name: &str) -> RenameParams {
    RenameParams {
        text_document: TextDocumentIdentifier { uri },
        position,
        new_name: new_name.into(),
        work_done_token: Default::default(),
    }
}

#[test]
fn invalid_new_name() {
    let uri = "untitled:test".to_string();
    let source = "";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 0, character: 0 }, "0")),
        Err("Invalid name `0`: not a valid identifier.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 0, character: 0 }, "abc")),
        Err("Invalid name `abc`: not a valid identifier.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 0, character: 0 }, "$")),
        Err("Invalid name `$`: not a valid identifier.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 0, character: 0 }, "$()")),
        Err("Invalid name `$()`: not a valid identifier.".into())
    );
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (call 0) ;; comment
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 1, character: 4 }, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 2, character: 29 }, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 3, character: 7 }, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 3, character: 18 }, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 4, character: 15 }, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), Position { line: 4, character: 23 }, "$f")),
        Err("This can't be renamed.".into())
    );
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func
        (call $func) (return_call $func)
    )
    (start $func)
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 14,
            },
            "$f",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn func_conflicts() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $func)
    (func $func)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 3,
                character: 14,
            },
            "$f",
        ))
        .unwrap();
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
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 14,
            },
            "$f",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i32)
        (local.get $param)
    )
    (func (param $param i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 21,
            },
            "$p",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i32)
        (local.get $local)
    )
    (func (local $local i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 21,
            },
            "$l",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn local_conflicts() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $name i64) (local $name i32)
        (local.get $name)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 39,
            },
            "$l",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func
        (call $func) (return_call $func)
    )
    (start $func)
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 5,
                character: 14,
            },
            "$f",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn param_access() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i32)
        (local.get $param)
    )
    (func (param $param i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 3,
                character: 21,
            },
            "$p",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn local_access() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i32)
        (local.get $local)
    )
    (func (local $local i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 3,
                character: 21,
            },
            "$l",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn func_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type)
    (func (type $type))
)
(module (type $type))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 14,
            },
            "$ty",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn type_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type)
    (func (type $type))
)
(module (type $type))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 3,
                character: 20,
            },
            "$ty",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn global_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global)
    (func (global.get $global))
)
(module (global $global))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 17,
            },
            "$g",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn global_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global)
    (func (global.get $global))
)
(module (global $global))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 3,
                character: 28,
            },
            "$g",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn memory_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (memory $memory (data))
    (export \"\" (memory $memory))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 17,
            },
            "$m",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn memory_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (memory $memory (data))
    (export \"\" (memory $memory))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 3,
                character: 28,
            },
            "$m",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn table_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 2,
                character: 13,
            },
            "$t",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn table_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 4,
                character: 21,
            },
            "$t",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn block_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $block
      (block
        (br_table $block))
      (br_table $block))
    (block $block
      (br_table $block))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 3,
                character: 16,
            },
            "$b",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn block_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $block
      (block
        (br_table $block))
      (br_table $block))
    (block $block
      (br_table $block))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .rename(create_params(
            uri,
            Position {
                line: 6,
                character: 21,
            },
            "$b",
        ))
        .unwrap();
    assert_json_snapshot!(response);
}
