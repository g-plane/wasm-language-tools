use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn cont_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont ))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 14));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont $))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont $f))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_sort() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft1 (func (param i32)))
  (type $ct1 (cont $ft1))
  (type $ft2 (func (param (ref null $ct3)) (result i32)))
  (type $ct2 (cont $ft2))
  (type $ft3 (func (result i32)))
  (type $ct3 (cont $)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 7, 20));
    assert_json_snapshot!(response);
}

#[test]
fn cont_new() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.new )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 17));
    assert_json_snapshot!(response);
}

#[test]
fn cont_new_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.new 2)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 18));
    assert_json_snapshot!(response);
}

#[test]
fn cont_new_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.new $)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 18));
    assert_json_snapshot!(response);
}

#[test]
fn cont_new_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.new $x)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 19));
    assert_json_snapshot!(response);
}

#[test]
fn cont_bind() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.bind $ct1 )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 23));
    assert_json_snapshot!(response);
}

#[test]
fn cont_bind_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.bind $ct1 2)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 24));
    assert_json_snapshot!(response);
}

#[test]
fn cont_bind_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.bind 2 $)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 21));
    assert_json_snapshot!(response);
}

#[test]
fn cont_bind_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func cont.bind 2 $x)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 22));
    assert_json_snapshot!(response);
}

#[test]
fn resume() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 15));
    assert_json_snapshot!(response);
}

#[test]
fn resume_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume $x)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 17));
    assert_json_snapshot!(response);
}

#[test]
fn resume_following_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume ()
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 16));
    assert_json_snapshot!(response);
}

#[test]
fn resume_following_paren_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func (resume ())
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 17));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw )
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 21));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw 1)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 22));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw $)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 22));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw $x)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 23));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_after_first() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw $x )
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 24));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_after_first_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw $x 1)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 25));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_after_first_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw $x $)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 25));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_after_first_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw $x $x)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 26));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_following_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw ()
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 22));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_ref_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw_ref 2)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 26));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_ref_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw_ref $)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 26));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_ref_following_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func resume_throw_ref ()
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 26));
    assert_json_snapshot!(response);
}

#[test]
fn resume_throw_ref_following_paren_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func (resume_throw_ref ())
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 27));
    assert_json_snapshot!(response);
}

#[test]
fn suspend() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func suspend )
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 16));
    assert_json_snapshot!(response);
}

#[test]
fn suspend_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func suspend 1)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 17));
    assert_json_snapshot!(response);
}

#[test]
fn suspend_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func suspend $)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 17));
    assert_json_snapshot!(response);
}

#[test]
fn suspend_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array i32))
  (type $ft1 (func))
  (type $ct1 (cont $ft1))
  (type (cont $ft1))
  (func suspend $x)
  (tag)
  (tag $exn)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 18));
    assert_json_snapshot!(response);
}
