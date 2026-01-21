use insta::assert_json_snapshot;
use lspt::{Position, TextDocumentIdentifier, TypeDefinitionParams};
use wat_service::LanguageService;

fn create_params(uri: String, line: u32, character: u32) -> TypeDefinitionParams {
    TypeDefinitionParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line, character },
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
    service.commit(&uri, source.into());
    assert!(service.goto_type_definition(create_params(uri.clone(), 1, 4)).is_none());
    assert!(
        service
            .goto_type_definition(create_params(uri.clone(), 2, 29))
            .is_none()
    );
    assert!(service.goto_type_definition(create_params(uri.clone(), 3, 7)).is_none());
    assert!(
        service
            .goto_type_definition(create_params(uri.clone(), 3, 25))
            .is_none()
    );
    assert!(
        service
            .goto_type_definition(create_params(uri.clone(), 4, 14))
            .is_none()
    );
    assert!(
        service
            .goto_type_definition(create_params(uri.clone(), 4, 23))
            .is_none()
    );
}

#[test]
fn func_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func
        (call 1)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    assert!(
        service
            .goto_type_definition(create_params(uri.clone(), 3, 15))
            .is_none()
    );
}

#[test]
fn type_use_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (type $type)
        (call 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    assert!(
        service
            .goto_type_definition(create_params(uri.clone(), 3, 15))
            .is_none()
    );
}

#[test]
fn func_int_idx_type_use_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (func))
    (func (type 0)
        (call 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 15));
    assert_json_snapshot!(response);
}

#[test]
fn func_ident_idx_type_use_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (func))
    (func $func (type 0)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 18));
    assert_json_snapshot!(response);
}

#[test]
fn func_int_idx_type_use_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func))
    (func (type $type)
        (call 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 15));
    assert_json_snapshot!(response);
}

#[test]
fn func_ident_idx_type_use_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func))
    (func $func (type $type)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 18));
    assert_json_snapshot!(response);
}

#[test]
fn param_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func (param (ref $s))
    local.get 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 15));
    assert_json_snapshot!(response);
}

#[test]
fn local_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct))
  (func (local $l (ref null 0))
    local.get $l))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 16));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func
    global.get 0)
  (global (ref null $s)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 16));
    assert_json_snapshot!(response);
}

#[test]
fn global_mut() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct))
  (func
    global.get $g)
  (global $g (mut (ref null 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.goto_type_definition(create_params(uri, 4, 16));
    assert_json_snapshot!(response);
}
