use insta::assert_json_snapshot;
use lspt::{
    Position, TextDocumentIdentifier, TypeHierarchyPrepareParams, TypeHierarchySubtypesParams,
    TypeHierarchySupertypesParams,
};
use wat_service::LanguageService;

const SOURCE: &str = "
(module
  (type $t1 (sub (struct)))
  (type $t2 (sub $t1 (struct)))
  (type $t3 (sub $t2 (struct)))
  (type $t4 (sub $t3 (struct)))
  (type $t5 (sub $t3 (struct))))
";

#[test]
fn t1_supertypes() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_type_hierarchy(TypeHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 2, character: 10 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let supertypes = service.type_hierarchy_supertypes(TypeHierarchySupertypesParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert!(supertypes.is_none());
}

#[test]
fn t1_subtypes() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_type_hierarchy(TypeHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 2, character: 10 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let subtypes = service.type_hierarchy_subtypes(TypeHierarchySubtypesParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(subtypes);
}

#[test]
fn t2_supertypes() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_type_hierarchy(TypeHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 4, character: 19 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let supertypes = service.type_hierarchy_supertypes(TypeHierarchySupertypesParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(supertypes);
}

#[test]
fn t3_subtypes() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_type_hierarchy(TypeHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 5, character: 19 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let subtypes = service.type_hierarchy_subtypes(TypeHierarchySubtypesParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(subtypes);
}

#[test]
fn t4_supertypes() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_type_hierarchy(TypeHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 5, character: 10 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let supertypes = service.type_hierarchy_supertypes(TypeHierarchySupertypesParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(supertypes);
}

#[test]
fn t5_subtypes() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_type_hierarchy(TypeHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 6, character: 10 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let subtypes = service.type_hierarchy_subtypes(TypeHierarchySubtypesParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert!(subtypes.unwrap().is_empty());
}
