use nosqo_base::{file_path::FilePath, result::NosqoResult};
use nosqo_model::StatementSet;
use nosqo_pal::pal::Pal;
use nosqo_parser::Parser;

/// Reads all ontology files, merges them into a single statement set, and
/// writes the rendered result to `target/ontology.nosqo`.
pub fn read_ontogies(pal: &dyn Pal) -> NosqoResult<StatementSet> {
    let ontology_directory = FilePath::from("knowledge/ontologies");
    let output_directory = FilePath::from("target");
    let output_path = output_directory.join("ontology.nosqo");

    let mut ontology_paths = pal
        .walk_directory(&ontology_directory, &[String::from("**/*.nosqo")])?
        .collect::<NosqoResult<Vec<_>>>()?;
    ontology_paths.sort();

    let mut statements = Vec::new();
    for ontology_path in ontology_paths {
        let content = pal.read_file_to_string(&ontology_path)?;
        let statement_set = Parser::parse_str(content.as_str())?;
        statements.extend_from_slice(&statement_set.statements);
    }

    let statement_set = StatementSet::new(statements);
    pal.create_directory_all(&output_directory)?;
    pal.write_file(&output_path, statement_set.to_nosqo_string().as_bytes())?;

    Ok(statement_set)
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use nosqo_pal::pal_mock::PalMock;

    use super::read_ontogies;

    #[test]
    fn reads_all_ontologies_and_writes_the_merged_output() {
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
            "knowledge/ontologies/a-person.nosqo",
            r#"
            #Person {
              isA #Type
              label "Person"
            }
            "#,
        );

        let statement_set = read_ontogies(&pal).unwrap();

        assert_eq!(statement_set.as_slice().len(), 4);
        expect![[r#"
            READ FILE: knowledge/ontologies/a-person.nosqo
            READ FILE: knowledge/ontologies/z-event.nosqo
            CREATE DIRECTORY: target
            WRITE FILE: target/ontology.nosqo -> #Event {
              isA #Type
              label "Event"
            }

            #Person {
              isA #Type
              label "Person"
            }
        "#]]
        .assert_eq(&pal.get_effects());

        assert_eq!(
            pal.read_file_string("target/ontology.nosqo").unwrap(),
            r#"
#Event {
  isA #Type
  label "Event"
}

#Person {
  isA #Type
  label "Person"
}"#
            .trim_start()
        );
    }
}
