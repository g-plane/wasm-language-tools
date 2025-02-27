use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn subsumption1() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f1 (sub (func))) (type (struct (field (ref $f1)))))
  (rec (type $f2 (sub (func))) (type (struct (field (ref $f2)))))
  (rec (type $g1 (sub $f1 (func))) (type (struct)))
  (rec (type $g2 (sub $f2 (func))) (type (struct)))
  (func (param (ref $g2)) (result (ref $g1))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn subsumption2() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f1 (sub (func))) (type $s1 (sub (struct (field (ref $f1))))))
  (rec (type $f2 (sub (func))) (type $s2 (sub (struct (field (ref $f2))))))
  (rec
    (type $g1 (sub $f1 (func)))
    (type
      (sub $s1 (struct
        (field (ref $f1) (ref $f1) (ref $f2) (ref $f2) (ref $g1))))))
  (rec
    (type $g2 (sub $f2 (func)))
    (type
      (sub $s2 (struct
        (field (ref $f1) (ref $f2) (ref $f1) (ref $f2) (ref $g2))))))
  (func (param (ref $g2)) (result (ref $g1))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn subsumption3() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f1 (sub (func))) (type (struct (field (ref $f1)))))
  (rec (type $f2 (sub (func))) (type (struct (field (ref $f1)))))
  (rec (type $g1 (sub $f1 (func))) (type (struct)))
  (rec (type $g2 (sub $f2 (func))) (type (struct)))
  (func (param (ref $g2)) (result (ref $g1))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn subsumption4() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f1 (sub (func))) (type (struct (field (ref $f1)))))
  (rec (type $f2 (sub (func))) (type (struct (field (ref $f2)))))
  (rec (type $g (sub $f1 (func))) (type (struct)))
  (func $g (type $g))
  (func (param (ref $g)) (result (ref $f1))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn subsumption5() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f1 (sub (func))) (type $s1 (sub (struct (field (ref $f1))))))
  (rec (type $f2 (sub (func))) (type $s2 (sub (struct (field (ref $f2))))))
  (rec
    (type $g1 (sub $f1 (func)))
    (type
      (sub $s1 (struct
        (field (ref $f1) (ref $f1) (ref $f2) (ref $f2) (ref $g1))))))
  (rec
    (type $g2 (sub $f2 (func)))
    (type
      (sub $s2 (struct
        (field (ref $f1) (ref $f2) (ref $f1) (ref $f2) (ref $g2))))))
  (rec (type $h (sub $g2 (func))) (type (struct)))
  (func $h (type $h))
  (func (param (ref $h)) (result (ref $f1))
    (local.get 0))
  (func (param (ref $h)) (result (ref $g1))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn subsumption6() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec
    (type $f11 (sub (func (result (ref func)))))
    (type $f12 (sub $f11 (func (result (ref $f11))))))
  (rec
    (type $f21 (sub (func (result (ref func)))))
    (type $f22 (sub $f21 (func (result (ref $f21))))))
  (func (param (ref $f11)) (result (ref $f11))
    (local.get 0))
  (func (param (ref $f11)) (result (ref $f21))
    (local.get 0))
  (func (param (ref $f12)) (result (ref $f12))
    (local.get 0))
  (func (param (ref $f12)) (result (ref $f22))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn subsumption7() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f11 (sub (func (result (ref func))))) (type $f12 (sub $f11 (func (result (ref $f11))))))
  (rec (type $f21 (sub (func (result (ref func))))) (type $f22 (sub $f21 (func (result (ref $f21))))))
  (rec (type $g11 (sub $f11 (func (result (ref func))))) (type $g12 (sub $g11 (func (result (ref $g11))))))
  (rec (type $g21 (sub $f21 (func (result (ref func))))) (type $g22 (sub $g21 (func (result (ref $g21))))))
  (func (param (ref $g11)) (result (ref $f11)) (local.get 0))
  (func (param (ref $g11)) (result (ref $f21)) (local.get 0))
  (func (param (ref $g12)) (result (ref $f11)) (local.get 0))
  (func (param (ref $g12)) (result (ref $f21)) (local.get 0))
  (func (param (ref $g11)) (result (ref $g11)) (local.get 0))
  (func (param (ref $g11)) (result (ref $g21)) (local.get 0))
  (func (param (ref $g12)) (result (ref $g12)) (local.get 0))
  (func (param (ref $g12)) (result (ref $g22)) (local.get 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn subsumption8() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f11 (sub (func))) (type $f12 (sub $f11 (func))))
  (rec (type $f21 (sub (func))) (type $f22 (sub $f11 (func))))
  (func (param (ref $f21)) (result (ref $f11))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn subsumption9() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $f01 (sub (func))) (type $f02 (sub $f01 (func))))
  (rec (type $f11 (sub (func))) (type $f12 (sub $f01 (func))))
  (rec (type $f21 (sub (func))) (type $f22 (sub $f11 (func))))
  (func (param (ref $f21)) (result (ref $f11))
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
