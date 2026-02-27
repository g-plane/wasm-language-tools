use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (func
    try_table (catch 0 0)
      throw 0
    end)
  (func
    suspend 0
    resume_throw 0 0 (on 0 0)
    switch 0 0))
(module
  (tag))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 5, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 5, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn def_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag $e)
  (func
    try_table (catch $e 0)
      throw $e
    end)
  (func
    suspend $e
    resume_throw $ct $e (on $e $label)
    switch $ct $e))
(module
  (tag $e))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 8, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 8, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn ref_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (func
    try_table (catch 0 0)
      throw 0
    end)
  (func
    suspend 0
    resume_throw 0 0 (on 0 0)
    switch 0 0))
(module
  (tag))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 21, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 21, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn ref_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag $e)
  (func
    try_table (catch $e 0)
      throw $e
    end)
  (func
    suspend $e
    resume_throw $ct $e (on $e $label)
    switch $ct $e))
(module
  (tag $e))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 13, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 13, false));
    assert_json_snapshot!(exclude_decl);
}
