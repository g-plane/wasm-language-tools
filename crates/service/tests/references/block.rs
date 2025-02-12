use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn block_def_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block
        (br_table 1))
      (br_table 0))
    (block
      (br_table 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 10), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 10), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_def_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $block
      (block
        (br_table $block))
      (br_table $block))
    (block $block
      (br_table $block))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 14), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 14), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_ref_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block
        (br_table 1))
      (br_table 0))
    (block
      (br_table 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(5, 19), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(5, 19), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_ref_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $block
      (block
        (br_table $block))
      (br_table $block))
    (block $block
      (br_table $block))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(5, 21), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(5, 21), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_relation() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $a
      (block $b
        br_table 0 1))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let def_a = service.find_references(create_params(uri.clone(), Position::new(3, 11), true));
    assert_json_snapshot!(def_a);
    let def_b = service.find_references(create_params(uri.clone(), Position::new(4, 14), true));
    assert_json_snapshot!(def_b);
    let ref_0 = service.find_references(create_params(uri.clone(), Position::new(5, 18), true));
    assert_json_snapshot!(ref_0);
    let ref_1 = service.find_references(create_params(uri.clone(), Position::new(5, 20), true));
    assert_json_snapshot!(ref_1);
}
