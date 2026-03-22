use std::collections::BTreeMap;

use nosqo_base::{err, result::NosqoResult};
use nosqo_model::{
    NqlBindingValue, NqlPattern, NqlQuery, NqlQueryResult, NqlReturn, NqlTerm, NqlVariable,
    Statement, StatementPattern, StatementPatternValue, Value,
};

use crate::StatementStore;

/// Executes an NQL query against a statement store.
pub fn execute_nql_query(
    store: &dyn StatementStore,
    query: &NqlQuery,
) -> NosqoResult<NqlQueryResult> {
    let mut rows = vec![BTreeMap::new()];

    for pattern in &query.patterns {
        let statement_pattern = statement_pattern_from_nql_pattern(pattern)?;
        let matching_statements = store.find_statements(&statement_pattern)?;
        let mut next_rows = Vec::new();

        for row in &rows {
            for statement in matching_statements.as_slice() {
                if let Some(next_row) = match_pattern(row, pattern, statement)? {
                    next_rows.push(next_row);
                }
            }
        }

        rows = next_rows;
    }

    let columns = projected_columns(query);
    let rows = rows
        .into_iter()
        .map(|row| {
            columns
                .iter()
                .map(|column| {
                    row.get(column).cloned().ok_or_else(|| {
                        err!(
                            "return variable `?{}` was not bound by the query",
                            column.as_str()
                        )
                    })
                })
                .collect::<NosqoResult<Vec<_>>>()
        })
        .collect::<NosqoResult<Vec<_>>>()?;

    Ok(NqlQueryResult::new(columns, rows))
}

fn projected_columns(query: &NqlQuery) -> Vec<NqlVariable> {
    match &query.return_spec {
        NqlReturn::All => collect_variables_in_first_appearance_order(query),
        NqlReturn::Variables(variables) => variables.clone(),
    }
}

fn collect_variables_in_first_appearance_order(query: &NqlQuery) -> Vec<NqlVariable> {
    let mut variables = Vec::new();

    for pattern in &query.patterns {
        collect_variable(&mut variables, &pattern.subject);
        collect_variable(&mut variables, &pattern.predicate);
        collect_variable(&mut variables, &pattern.object);
    }

    variables
}

fn collect_variable(variables: &mut Vec<NqlVariable>, term: &NqlTerm) {
    let NqlTerm::Variable(variable) = term else {
        return;
    };

    if !variables.contains(variable) {
        variables.push(variable.clone());
    }
}

fn statement_pattern_from_nql_pattern(pattern: &NqlPattern) -> NosqoResult<StatementPattern> {
    Ok(StatementPattern::new(
        subject_pattern_value(&pattern.subject)?,
        predicate_pattern_value(&pattern.predicate)?,
        object_pattern_value(&pattern.object)?,
    ))
}

fn subject_pattern_value(
    term: &NqlTerm,
) -> NosqoResult<StatementPatternValue<nosqo_model::NodeId>> {
    match term {
        NqlTerm::Variable(_) => Ok(StatementPatternValue::Any),
        NqlTerm::Id(id) => Ok(StatementPatternValue::Exact(id.clone())),
        NqlTerm::Value(_) => Err(err!("subject terms must be variables or identifiers")),
    }
}

fn predicate_pattern_value(
    term: &NqlTerm,
) -> NosqoResult<StatementPatternValue<nosqo_model::NodeId>> {
    match term {
        NqlTerm::Variable(_) => Ok(StatementPatternValue::Any),
        NqlTerm::Id(id) => Ok(StatementPatternValue::Exact(id.clone())),
        NqlTerm::Value(_) => Err(err!("predicate terms must be variables or identifiers")),
    }
}

fn object_pattern_value(term: &NqlTerm) -> NosqoResult<StatementPatternValue<Value>> {
    match term {
        NqlTerm::Variable(_) => Ok(StatementPatternValue::Any),
        NqlTerm::Id(id) => Ok(StatementPatternValue::Exact(Value::Id(id.clone()))),
        NqlTerm::Value(value) => Ok(StatementPatternValue::Exact(value.clone())),
    }
}

fn match_pattern(
    row: &BTreeMap<NqlVariable, NqlBindingValue>,
    pattern: &NqlPattern,
    statement: &Statement,
) -> NosqoResult<Option<BTreeMap<NqlVariable, NqlBindingValue>>> {
    let mut next_row = row.clone();

    if !match_id_term(&mut next_row, &pattern.subject, &statement.subject)? {
        return Ok(None);
    }

    if !match_id_term(&mut next_row, &pattern.predicate, &statement.predicate)? {
        return Ok(None);
    }

    if !match_value_term(&mut next_row, &pattern.object, &statement.object) {
        return Ok(None);
    }

    Ok(Some(next_row))
}

fn match_id_term(
    row: &mut BTreeMap<NqlVariable, NqlBindingValue>,
    term: &NqlTerm,
    value: &nosqo_model::NodeId,
) -> NosqoResult<bool> {
    match term {
        NqlTerm::Variable(variable) => Ok(bind_variable(row, variable, value.clone().into())),
        NqlTerm::Id(expected) => Ok(expected == value),
        NqlTerm::Value(_) => Err(err!(
            "subject and predicate terms must not be literal values"
        )),
    }
}

fn match_value_term(
    row: &mut BTreeMap<NqlVariable, NqlBindingValue>,
    term: &NqlTerm,
    value: &Value,
) -> bool {
    match term {
        NqlTerm::Variable(variable) => bind_variable(row, variable, value.clone().into()),
        NqlTerm::Id(expected) => value == &Value::Id(expected.clone()),
        NqlTerm::Value(expected) => expected == value,
    }
}

fn bind_variable(
    row: &mut BTreeMap<NqlVariable, NqlBindingValue>,
    variable: &NqlVariable,
    value: NqlBindingValue,
) -> bool {
    match row.get(variable) {
        Some(existing) => existing == &value,
        None => {
            row.insert(variable.clone(), value);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use nosqo_model::{Statement, StatementSet};

    use super::execute_nql_query;
    use crate::{InMemoryStatementStore, StatementStore};

    #[test]
    fn executes_data_driven_query_cases() {
        struct Case {
            name: &'static str,
            query: &'static str,
            expected_columns: Vec<&'static str>,
            expected_rows: Vec<Vec<&'static str>>,
        }

        let store = seeded_store();
        let cases = vec![
            Case {
                name: "single pattern",
                query: "match\n?city ~label \"Berlin\"\nreturn\n?city\n",
                expected_columns: vec!["city"],
                expected_rows: vec![vec!["@berlin"]],
            },
            Case {
                name: "multi pattern join",
                query: "match\n?city ~isA #City\n?city ~label ?label\nreturn\n?city ?label\n",
                expected_columns: vec!["city", "label"],
                expected_rows: vec![vec!["@berlin", "\"Berlin\""], vec!["@paris", "\"Paris\""]],
            },
            Case {
                name: "repeated variable unification",
                query: "match\n?city ~capitalOf ?country\n?country ~capital ?city\nreturn\n?city ?country\n",
                expected_columns: vec!["city", "country"],
                expected_rows: vec![vec!["@berlin", "@germany"]],
            },
            Case {
                name: "return all variables follows first appearance order",
                query: "match\n?city ~capitalOf ?country\n?country ~label ?label\nreturn\n*\n",
                expected_columns: vec!["city", "country", "label"],
                expected_rows: vec![vec!["@berlin", "@germany", "\"Germany\""]],
            },
            Case {
                name: "empty result set",
                query: "match\n?city ~label \"Rome\"\nreturn\n?city\n",
                expected_columns: vec!["city"],
                expected_rows: vec![],
            },
        ];

        for case in cases {
            let query = nosqo_parser::NqlParser::parse_str(case.query)
                .unwrap_or_else(|error| panic!("{} should parse: {}", case.name, error.kind()));
            let result = execute_nql_query(&store, &query)
                .unwrap_or_else(|error| panic!("{} should execute: {}", case.name, error.kind()));

            assert_eq!(
                result
                    .columns
                    .iter()
                    .map(|column| column.as_str().to_string())
                    .collect::<Vec<_>>(),
                case.expected_columns
                    .into_iter()
                    .map(str::to_string)
                    .collect::<Vec<_>>(),
                "{} should preserve expected columns",
                case.name
            );
            assert_eq!(
                result
                    .rows
                    .iter()
                    .map(|row| row
                        .iter()
                        .map(|value| value.to_nosqo_string())
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                case.expected_rows
                    .into_iter()
                    .map(|row| row.into_iter().map(str::to_string).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                "{} should preserve expected rows",
                case.name
            );
        }
    }

    fn seeded_store() -> InMemoryStatementStore {
        let store = InMemoryStatementStore::default();
        store
            .assert_statements(StatementSet::from(vec![
                Statement::from_strings("berlin", "isA", "#City"),
                Statement::from_strings("berlin", "label", "Berlin"),
                Statement::from_strings("berlin", "capitalOf", "@germany"),
                Statement::from_strings("paris", "isA", "#City"),
                Statement::from_strings("paris", "label", "Paris"),
                Statement::from_strings("germany", "label", "Germany"),
                Statement::from_strings("germany", "capital", "@berlin"),
            ]))
            .expect("test store should accept seed statements");
        store
    }
}
