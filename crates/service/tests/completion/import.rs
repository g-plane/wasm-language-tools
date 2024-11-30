use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn desc_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (import ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 13)));
    assert_json_snapshot!(response);
}

#[test]
fn table_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (import "" "" (table ))
)
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn global_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (import "" "" (global ))
)
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 26)));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_after_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (import "" "" (global ()))
)
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 27)));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_after_mut() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (import "" "" (global (mut )))
)
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 31)));
    assert_json_snapshot!(response);
}

#[test]
fn type_use() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (import "" "" (func ()))
)
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 25)));
    assert_json_snapshot!(response);
}