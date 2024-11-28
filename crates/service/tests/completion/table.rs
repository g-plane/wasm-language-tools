use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn top_level() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (table )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn after_top_level_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (table ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 12)));
    assert_json_snapshot!(response);
}

#[test]
fn table_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (table 0 )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 13)));
    assert_json_snapshot!(response);
}

#[test]
fn elem_without_parens() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $f)
    (table funcref (elem ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn elem_with_parens() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (table funcref (elem ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 26)));
    assert_json_snapshot!(response);
}

#[test]
fn instr_after_item() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (table funcref (elem (item )))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 31)));
    assert_json_snapshot!(response);
}

#[test]
fn instr_after_item_and_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (table funcref (elem (item ())))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 32)));
    assert_json_snapshot!(response);
}
