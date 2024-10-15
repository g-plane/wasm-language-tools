use super::*;
use insta::assert_json_snapshot;
use lsp_types::{GotoDefinitionResponse, Position, Uri};
use wat_service::LanguageService;

#[test]
fn func_not_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call 1) (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    assert!(matches!(
        service.goto_definition(create_params(uri.clone(), Position::new(3, 15))),
        Some(GotoDefinitionResponse::Array(locations)) if locations.is_empty()
    ));
    assert!(matches!(
        service.goto_definition(create_params(uri.clone(), Position::new(3, 25))),
        Some(GotoDefinitionResponse::Array(locations)) if locations.is_empty()
    ));
}

#[test]
fn func_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 15)));
    assert_json_snapshot!(response);
}

#[test]
fn func_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn param_or_local_not_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i64) (local $local i64))
    (func
        (local.get 0) (local.get $param) (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    assert!(service
        .goto_definition(create_params(uri.clone(), Position::new(4, 20)))
        .is_none());
    assert!(service
        .goto_definition(create_params(uri.clone(), Position::new(4, 37)))
        .is_none());
    assert!(service
        .goto_definition(create_params(uri, Position::new(4, 57)))
        .is_none());
}

#[test]
fn param_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param v128)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn param_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param v128)
        (local.get $param)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn local_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local v128)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn local_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $local v128)
        (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_not_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (type 0))
    (func (type $type))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    assert!(matches!(
        service.goto_definition(create_params(uri.clone(), Position::new(2, 17))),
        Some(GotoDefinitionResponse::Array(locations)) if locations.is_empty()
    ));
    assert!(matches!(
        service.goto_definition(create_params(uri.clone(), Position::new(3, 18))),
        Some(GotoDefinitionResponse::Array(locations)) if locations.is_empty()
    ));
}

#[test]
fn type_use_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func))
    (func (type 0))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func))
    (func (type $type))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(3, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn global_not_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (global.get 0) (global.get $global)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    assert!(matches!(
        service.goto_definition(create_params(uri.clone(), Position::new(3, 21))),
        Some(GotoDefinitionResponse::Array(locations)) if locations.is_empty()
    ));
    assert!(matches!(
        service.goto_definition(create_params(uri.clone(), Position::new(3, 40))),
        Some(GotoDefinitionResponse::Array(locations)) if locations.is_empty()
    ));
}

#[test]
fn global_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global i32)
    (func
        (global.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(4, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn global_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global i32)
    (func
        (global.get $global)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.goto_definition(create_params(uri, Position::new(4, 26)));
    assert_json_snapshot!(response);
}
