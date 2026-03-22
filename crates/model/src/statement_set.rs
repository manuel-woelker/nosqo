use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{NodeId, Statement};

/// A collection of statements.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StatementSet {
    /// The statements in the set.
    pub statements: Vec<Statement>,
}

impl StatementSet {
    /// Creates a statement set from a vector of statements.
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    /// Returns the statements as a slice.
    pub fn as_slice(&self) -> &[Statement] {
        &self.statements
    }

    /// Appends a statement to the set.
    pub fn push(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    /// Renders the statement set as nosqo text using subject blocks.
    pub fn to_nosqo_string(&self) -> String {
        let mut rendered = String::new();
        let subject_groups = self.grouped_by_subject();

        for (index, (subject, statements)) in subject_groups.iter().enumerate() {
            if index > 0 {
                rendered.push('\n');
                rendered.push('\n');
            }

            rendered.push_str(&format_subject(subject));
            rendered.push_str(" {\n");

            for statement in statements {
                rendered.push_str("  ");
                rendered.push_str(&format_predicate(&statement.predicate));
                rendered.push(' ');
                rendered.push_str(&statement.object.to_nosqo_string());
                rendered.push('\n');
            }

            rendered.push('}');
        }

        rendered
    }

    fn grouped_by_subject(&self) -> Vec<(NodeId, Vec<&Statement>)> {
        let mut groups: BTreeMap<NodeId, Vec<&Statement>> = BTreeMap::new();

        for statement in &self.statements {
            groups
                .entry(statement.subject.clone())
                .or_default()
                .push(statement);
        }

        groups
            .into_iter()
            .map(|(subject, mut statements)| {
                statements.sort_by(|left, right| {
                    left.predicate
                        .cmp(&right.predicate)
                        .then_with(|| left.object.cmp(&right.object))
                });
                (subject, statements)
            })
            .collect()
    }
}

impl From<Vec<Statement>> for StatementSet {
    fn from(statements: Vec<Statement>) -> Self {
        Self::new(statements)
    }
}

fn format_subject(subject: &NodeId) -> String {
    subject.as_str().to_string()
}

fn format_predicate(predicate: &NodeId) -> String {
    predicate.as_str().trim_start_matches('~').to_string()
}

#[cfg(test)]
mod tests {
    use super::StatementSet;
    use crate::Statement;

    #[test]
    fn creates_statement_sets_from_vectors() {
        let statements = vec![Statement::from_strings("berlin", "label", "Berlin")];

        let statement_set = StatementSet::new(statements.clone());

        assert_eq!(statement_set.as_slice(), statements.as_slice());
    }

    #[test]
    fn appends_statements() {
        let mut statement_set = StatementSet::default();
        let statement = Statement::from_strings("berlin", "isA", "#City");

        statement_set.push(statement.clone());

        assert_eq!(statement_set.as_slice(), &[statement]);
    }

    #[test]
    fn renders_subject_blocks_in_alphabetical_order() {
        let statement_set = StatementSet::new(vec![
            Statement::from_strings("paris", "isA", "#City"),
            Statement::from_strings("berlin", "label", "Berlin"),
            Statement::from_strings("berlin", "isA", "#City"),
        ]);

        assert_eq!(
            statement_set.to_nosqo_string(),
            r#"
berlin {
  isA #City
  label "Berlin"
}

paris {
  isA #City
}"#
            .trim_start()
        );
    }

    #[test]
    fn renders_predicates_and_values_in_alphabetical_order() {
        let statement_set = StatementSet::new(vec![
            Statement::from_strings("berlin", "speaks", "'fr'"),
            Statement::from_strings("berlin", "label", "Berlin"),
            Statement::from_strings("berlin", "speaks", "'de'"),
            Statement::from_strings("berlin", "capitalOf", "@germany"),
        ]);

        assert_eq!(
            statement_set.to_nosqo_string(),
            r#"
berlin {
  capitalOf @germany
  label "Berlin"
  speaks 'de'
  speaks 'fr'
}"#
            .trim_start()
        );
    }

    #[test]
    fn renders_object_ids_and_escaped_literals() {
        let statement_set = StatementSet::new(vec![
            Statement::from_strings("berlin", "label", "The \"Capital\"\\n"),
            Statement::from_strings("berlin", "symbol", "'it\\'s_fine'"),
        ]);

        assert_eq!(
            statement_set.to_nosqo_string(),
            r#"
berlin {
  label "The \"Capital\"\\n"
  symbol 'it\'s_fine'
}"#
            .trim_start()
        );
    }
}
