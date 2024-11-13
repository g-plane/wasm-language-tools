use insta::assert_json_snapshot;
use lsp_types::{DocumentFormattingParams, FormattingOptions, TextDocumentIdentifier, Uri};
use wat_service::LanguageService;

fn create_params(uri: Uri, options: FormattingOptions) -> DocumentFormattingParams {
    DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        options,
        work_done_progress_params: Default::default(),
    }
}

#[test]
fn space2() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i32)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32)
    (local.get 0)
  )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32)
    (local.get 0)
  )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
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
