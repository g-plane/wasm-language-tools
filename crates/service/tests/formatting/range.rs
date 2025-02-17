use insta::assert_json_snapshot;
use lspt::{
    DocumentRangeFormattingParams, FormattingOptions, Position, Range, TextDocumentIdentifier, Uri,
};
use wat_service::{LanguageService, ServiceConfig};

fn create_params(uri: String, range: Range) -> DocumentRangeFormattingParams {
    DocumentRangeFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        range,
        options: FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_token: Default::default(),
    }
}

#[test]
fn fully_covered_node() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    ( local.get    0 )
  )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.range_formatting(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 4,
            },
            end: Position {
                line: 3,
                character: 22,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn overlap() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
(block $b
    (block     $a
            (block   $b
          (block $c
            (block $b   )

          )

        )
      )
)
    ))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.range_formatting(create_params(
        uri,
        Range {
            start: Position {
                line: 4,
                character: 8,
            },
            end: Position {
                line: 5,
                character: 23,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn format_comments() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ;;comment
  )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    service.set_config(
        uri.clone(),
        ServiceConfig {
            format: wat_formatter::config::LanguageOptions {
                format_comments: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );
    let response = service.range_formatting(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 4,
            },
            end: Position {
                line: 3,
                character: 13,
            },
        },
    ));
    assert_json_snapshot!(response);
}
