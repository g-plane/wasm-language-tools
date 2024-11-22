use insta::assert_json_snapshot;
use lsp_types::{FoldingRangeParams, TextDocumentIdentifier, Uri};
use wat_service::LanguageService;

fn create_params(uri: Uri) -> FoldingRangeParams {
    FoldingRangeParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    }
}

#[test]
fn with_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    local.get 0
    if (result f64)
      f64.const 1
    else
      local.get 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.folding_range(create_params(uri));
    assert_json_snapshot!(response);
}
