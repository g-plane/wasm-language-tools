use super::*;
use insta::assert_json_snapshot;
use lspt::{Diagnostic, NumberOrString, StringOrMarkupContent};
use wat_service::LanguageService;

#[test]
fn leading_whitespace() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.load align =1))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 25, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn trailing_whitespace() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.load align= 1))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 26, 2, 26));
    assert_json_snapshot!(response);
}

#[test]
fn around_whitespaces() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.load align = 1))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 25, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn sequence_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func i32.load align = 1 drop)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 25, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn diagnostics() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.load align =1))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let mut params = create_params(uri, 2, 25, 2, 25);
    params.context = CodeActionContext {
        diagnostics: vec![Diagnostic {
            range: Range::default(),
            severity: None,
            code: Some(NumberOrString::String("syntax".into())),
            code_description: None,
            source: None,
            message: StringOrMarkupContent::String(
                "syntax error: whitespaces or comments are not allowed inside memory argument".into(),
            ),
            tags: None,
            related_information: None,
            data: None,
        }],
        ..Default::default()
    };
    let response = service.code_action(params);
    assert_json_snapshot!(response);
}
