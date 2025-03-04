use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn implicit_final() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func))
  (type $s (sub $t (func))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn explicit_final() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (sub final (func)))
  (type $s (sub $t (func))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn explicit_final_with_super() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (sub (func)))
  (type $s (sub final $t (func)))
  (type $u (sub $s (func))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn sub_type_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a0 (sub (array i32)))
  (type $s0 (sub $a0 (struct)))

  (type $f1 (sub (func (param i32) (result i32))))
  (type $s1 (sub $f1 (struct)))

  (type $s2 (sub (struct)))
  (type $a2 (sub $s2 (array i32)))

  (type $f3 (sub (func (param i32) (result i32))))
  (type $a3 (sub $f3 (array i32)))

  (type $s4 (sub (struct)))
  (type $f4 (sub $s4 (func (param i32) (result i32))))

  (type $a5 (sub (array i32)))
  (type $f5 (sub $a5 (func (param i32) (result i32))))

  (type $a6 (sub (array i32)))
  (type $a6' (sub $a6 (array i64)))

  (type $s7 (sub (struct (field i32))))
  (type $s7' (sub $s7 (struct (field i64))))

  (type $f8 (sub (func)))
  (type $f8' (sub $f8 (func (param i32)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn fields_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  ;; When fields are mutable, a subtype's reference fields cannot be subtypes of
  ;; the supertype's fields, they must match exactly.
  (type $a (sub (struct (field (mut (ref null any))))))
  (type $b (sub $a (struct (field (mut (ref null none))))))

  ;; When fields are const, a subtype's reference fields cannot be supertypes of
  ;; the supertype's fields, they must be subtypes.
  (type $c (sub (struct (field (ref null none)))))
  (type $d (sub $c (struct (field (ref null any)))))

  ;; The mutability of fields must be the same.
  (type $e (sub (struct (field (mut (ref null any))))))
  (type $f (sub $e (struct (field (ref null any)))))

  ;; The mutability of fields must be the same.
  (type $g (sub (struct (field (ref null any)))))
  (type $h (sub $g (struct (field (mut (ref null any)))))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (sub 1 (func)))
  (type (sub (func)))

  (type $z (sub $a (func)))
  (type $a (sub (func))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
