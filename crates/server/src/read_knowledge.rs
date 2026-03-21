use nosqo_base::{
    file_path::FilePath,
    result::{NosqoResult, ResultExt},
};
use nosqo_engine::{InMemoryStatementStore, StatementStore};
use nosqo_pal::pal::Pal;
use nosqo_parser::Parser;

/// Reads all `.nosqo` files under `knowledge/` into an in-memory statement
/// store.
pub fn read_knowledge(pal: &dyn Pal) -> NosqoResult<InMemoryStatementStore> {
    let knowledge_directory = FilePath::from("knowledge");
    let mut knowledge_paths = pal
        .walk_directory(&knowledge_directory, &[String::from("**/*.nosqo")])?
        .collect::<NosqoResult<Vec<_>>>()?;
    knowledge_paths.sort();

    let store = InMemoryStatementStore::default();
    for knowledge_path in knowledge_paths {
        let content = pal
            .read_file_to_string(&knowledge_path)
            .with_context(|| format!("failed to read knowledge file `{knowledge_path}`"))?;
        let statement_set = Parser::parse_str(content.as_str())
            .with_context(|| format!("failed to parse knowledge file `{knowledge_path}`"))?;
        store.assert_statements(statement_set)?;
    }

    Ok(store)
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use nosqo_engine::StatementStore;
    use nosqo_model::{Statement, StatementPattern};
    use nosqo_pal::pal_mock::PalMock;

    use super::read_knowledge;

    #[test]
    fn reads_all_nosqo_files_under_knowledge_into_the_store() {
        let pal = PalMock::new();
        pal.set_file(
            "knowledge/ontologies/z-event.nosqo",
            r#"
            #Event {
              isA #Type
              label "Event"
            }
            "#,
        );
        pal.set_file(
            "knowledge/a-person.nosqo",
            r#"
            @alice label "Alice"
            "#,
        );

        let store = read_knowledge(&pal).unwrap();
        let statement_set = store.find_statements(&StatementPattern::any()).unwrap();

        assert_eq!(statement_set.as_slice().len(), 3);
        assert!(
            statement_set
                .as_slice()
                .contains(&Statement::from_strings("alice", "label", "Alice"))
        );
        assert!(
            statement_set
                .as_slice()
                .contains(&Statement::from_strings("#Event", "isA", "#Type"))
        );
        expect![[r#"
            READ FILE: knowledge/a-person.nosqo
            READ FILE: knowledge/ontologies/z-event.nosqo
        "#]]
        .assert_eq(&pal.get_effects());
    }

    #[test]
    fn adds_file_context_when_parsing_fails() {
        let pal = PalMock::new();
        pal.set_file(
            "knowledge/broken.nosqo",
            r#"
            broken predicate {
            "#,
        );

        let error = match read_knowledge(&pal) {
            Ok(_) => panic!("expected knowledge loading to fail for invalid input"),
            Err(error) => error,
        };
        let rendered = error.to_test_string();

        assert!(rendered.contains("failed to parse knowledge file `knowledge/broken.nosqo`"));
        assert!(rendered.contains("expected an object value"));
    }
}
