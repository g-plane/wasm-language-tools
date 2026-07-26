use super::create_params;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn before_annotations() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    ;;; ## Function with Annotations
    (@custom (x))
    (func $func (param $param i32) (param f32 f64) (result i32 i64)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 12));
    assert_json_snapshot!(response);
}

#[test]
fn after_annotations() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (@custom (x))
    ;;; ## Function with Annotations
    (func $func (param $param i32) (param f32 f64) (result i32 i64)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 12));
    assert_json_snapshot!(response);
}
