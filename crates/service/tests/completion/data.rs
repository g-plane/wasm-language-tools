use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn module_field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 11,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn memory_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (me))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 13,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn memory_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (memory ))
    (memory $m 1)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn offset_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (of))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 13,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (offset ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn instr_inside_parens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (offset ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 19,
        },
    ));
    assert_json_snapshot!(response);
}
