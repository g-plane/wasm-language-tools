use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn block_def_int_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 10, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 10, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_def_ident_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 14, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 14, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_ref_int_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(uri.clone(), 5, 19, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 19, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_ref_ident_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(uri.clone(), 5, 21, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 21, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_relation() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $a
      (block $b
        br_table 0 1))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let def_a = service.find_references(create_params(uri.clone(), 3, 11, true));
    assert_json_snapshot!(def_a);
    let def_b = service.find_references(create_params(uri.clone(), 4, 14, true));
    assert_json_snapshot!(def_b);
    let ref_0 = service.find_references(create_params(uri.clone(), 5, 18, true));
    assert_json_snapshot!(ref_0);
    let ref_1 = service.find_references(create_params(uri.clone(), 5, 20, true));
    assert_json_snapshot!(ref_1);
}

#[test]
fn try_table() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    try_table $try (catch $e 0) (catch_ref $e 0) (catch_all $try) (catch_all_ref $try)
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let def = service.find_references(create_params(uri.clone(), 3, 16, true));
    assert_json_snapshot!(def);
    let ref_1 = service.find_references(create_params(uri.clone(), 3, 46, true));
    assert_json_snapshot!(ref_1);
}
