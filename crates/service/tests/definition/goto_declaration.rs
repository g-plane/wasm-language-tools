use super::*;
use insta::assert_json_snapshot;
use lsp_types::{GotoDefinitionResponse, Position, Uri};
use wat_service::LanguageService;

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (cal 0) ;; typo
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    assert!(service
        .goto_declaration(create_params(uri.clone(), Position::new(1, 4)))
        .is_none());
    assert!(service
        .goto_declaration(create_params(uri.clone(), Position::new(2, 29)))
        .is_none());
    assert!(service
        .goto_declaration(create_params(uri.clone(), Position::new(3, 7)))
        .is_none());
    assert!(service
        .goto_declaration(create_params(uri.clone(), Position::new(3, 25)))
        .is_none());
    assert!(service
        .goto_declaration(create_params(uri.clone(), Position::new(4, 14)))
        .is_none());
    assert!(service
        .goto_declaration(create_params(uri.clone(), Position::new(4, 23)))
        .is_none());
}

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
        service.goto_declaration(create_params(uri.clone(), Position::new(3, 15))),
        Some(GotoDefinitionResponse::Array(locations)) if locations.is_empty()
    ));
    assert!(matches!(
        service.goto_declaration(create_params(uri.clone(), Position::new(3, 25))),
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
    let response = service.goto_declaration(create_params(uri, Position::new(3, 15)));
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
    let response = service.goto_declaration(create_params(uri, Position::new(3, 18)));
    assert_json_snapshot!(response);
}
