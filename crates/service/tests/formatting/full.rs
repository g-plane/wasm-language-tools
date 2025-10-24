use insta::assert_json_snapshot;
use lspt::{DocumentFormattingParams, FormattingOptions, TextDocumentIdentifier};
use wat_service::{LanguageService, ServiceConfig};

fn create_params(uri: String, options: FormattingOptions) -> DocumentFormattingParams {
    DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        options,
        work_done_token: Default::default(),
    }
}

#[test]
fn space2() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.formatting(create_params(
        uri,
        FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn space4() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    (local.get 0)
  )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.formatting(create_params(
        uri,
        FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn tab() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    (local.get 0)
  )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.formatting(create_params(
        uri,
        FormattingOptions {
            tab_size: 2,
            insert_spaces: false,
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn format_comments() {
    let uri = "untitled:test".to_string();
    let source = ";;comment";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    service.set_config(
        uri.clone(),
        Some(ServiceConfig {
            format: wat_formatter::config::LanguageOptions {
                format_comments: true,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
    let response = service.formatting(create_params(
        uri,
        FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}
