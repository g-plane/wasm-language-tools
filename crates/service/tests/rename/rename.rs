use insta::assert_json_snapshot;
use lspt::{Position, RenameParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, line: u32, character: u32, new_name: &str) -> RenameParams {
    RenameParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line, character },
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
        service.rename(create_params(uri.clone(), 0, 0, "0")),
        Err("Invalid name `0`: not a valid identifier.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 0, 0, "abc")),
        Err("Invalid name `abc`: not a valid identifier.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 0, 0, "$")),
        Err("Invalid name `$`: not a valid identifier.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 0, 0, "$()")),
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
        service.rename(create_params(uri.clone(), 1, 4, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 2, 29, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 3, 7, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 3, 18, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 4, 15, "$f")),
        Err("This can't be renamed.".into())
    );
    assert_eq!(
        service.rename(create_params(uri.clone(), 4, 23, "$f")),
        Err("This can't be renamed.".into())
    );
}

#[test]
fn different_kinds() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func)
  (type $func (func)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 12, "$f")).unwrap();
    assert_json_snapshot!(response);
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
    let response = service.rename(create_params(uri, 2, 14, "$f")).unwrap();
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
    let response = service.rename(create_params(uri, 2, 14, "$f")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i32) (param $param' i32)
        (local.get $param)
    )
    (func (param $param i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 21, "$p")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i32) (local $local' i32)
        (local.get $local)
    )
    (func (local $local i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 21, "$l")).unwrap();
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
    let response = service.rename(create_params(uri, 2, 39, "$l")).unwrap();
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
    (func $func')
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 5, 14, "$f")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn param_access() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i32) (param $param' i32)
        (local.get $param)
    )
    (func (param $param i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 3, 21, "$p")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn local_access() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i32) (local $local' i32)
        (local.get $local)
    )
    (func (local $local i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 3, 21, "$l")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn func_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type)
    (func (type $type))
    (type $type')
)
(module (type $type))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 14, "$ty")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn type_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type)
    (func (type $type))
    (type $type')
)
(module (type $type))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 3, 20, "$ty")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn global_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global)
    (func (global.get $global))
    (global $global')
)
(module (global $global))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 17, "$g")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn global_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global)
    (func (global.get $global))
    (global $global')
)
(module (global $global))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 3, 28, "$g")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn memory_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (memory $memory (data))
    (export \"\" (memory $memory))
    (memory $memory' (data))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 17, "$m")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn memory_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (memory $memory (data))
    (export \"\" (memory $memory))
    (memory $memory' (data))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 3, 28, "$m")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn table_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table))
  (table $table' 0 funcref))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 13, "$t")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn table_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table))
  (table $table' 0 funcref))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 4, 21, "$t")).unwrap();
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
    let response = service.rename(create_params(uri, 3, 16, "$b")).unwrap();
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
    let response = service.rename(create_params(uri, 6, 21, "$b")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn field_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32)))
  (type (struct (field $x i32)))
  (func (param (ref 0)) (result i32)
    (struct.get 0 $x
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 2, 24, "$y")).unwrap();
    assert_json_snapshot!(response);
}

#[test]
fn field_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32)))
  (type (struct (field $x i32)))
  (func (param (ref 0)) (result i32)
    (struct.get 0 $x
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.rename(create_params(uri, 5, 19, "$y")).unwrap();
    assert_json_snapshot!(response);
}
