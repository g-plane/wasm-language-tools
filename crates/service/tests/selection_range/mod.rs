use insta::assert_json_snapshot;
use lsp_types::{Position, SelectionRangeParams, TextDocumentIdentifier, Uri};
use wat_service::LanguageService;

fn create_params(uri: Uri, positions: Vec<Position>) -> SelectionRangeParams {
    SelectionRangeParams {
        text_document: TextDocumentIdentifier { uri },
        positions,
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    }
}

#[test]
fn sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func
    local.get 1
    if
      local.get 2
    else
      local.get 3
    end)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.selection_range(create_params(uri, vec![Position::new(7, 17)]));
    assert_json_snapshot!(response);
}
