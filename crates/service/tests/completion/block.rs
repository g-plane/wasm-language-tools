use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn blocks() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table ))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(8, 24)));
    assert_json_snapshot!(response);
}

#[test]
fn blocks_following_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table 1))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(8, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn blocks_following_dollar() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table $))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(8, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn blocks_following_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table $a))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(8, 26)));
    assert_json_snapshot!(response);
}

#[test]
fn multiple_indexes() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table $b 1 ))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(8, 29)));
    assert_json_snapshot!(response);
}
