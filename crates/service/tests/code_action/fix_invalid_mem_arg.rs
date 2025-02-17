use super::*;
use insta::assert_json_snapshot;
use lspt::{Diagnostic, Position, Range, Union2};
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
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 25,
            },
            end: Position {
                line: 2,
                character: 25,
            },
        },
    ));
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
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 26,
            },
            end: Position {
                line: 2,
                character: 26,
            },
        },
    ));
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
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 25,
            },
            end: Position {
                line: 2,
                character: 25,
            },
        },
    ));
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
    service.commit(uri.clone(), source.into());
    let mut params = create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 25,
            },
            end: Position {
                line: 2,
                character: 25,
            },
        },
    );
    params.context = CodeActionContext {
        diagnostics: vec![Diagnostic {
            message: "syntax error: expected".into(),
            code: Some(Union2::B("syntax/".into())),
            ..Default::default()
        }],
        ..Default::default()
    };
    let response = service.code_action(params);
    assert_json_snapshot!(response);
}
