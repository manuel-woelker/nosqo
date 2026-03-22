use nosqo_base::{err, result::NosqoResult};
use nosqo_model::{NqlQuery, NqlReturn, NqlTerm, NqlVariable};

/// Validates NQL query rules that the parser does not enforce structurally.
pub fn validate_nql_query(query: &NqlQuery) -> NosqoResult<()> {
    for pattern in &query.patterns {
        validate_subject_or_predicate_term(&pattern.subject, "subject")?;
        validate_subject_or_predicate_term(&pattern.predicate, "predicate")?;
    }

    if let NqlReturn::Variables(variables) = &query.return_spec {
        let bound_variables = bound_variables(query);
        for variable in variables {
            if !bound_variables.contains(variable) {
                return Err(err!(
                    "return variable `?{}` does not appear in the match block",
                    variable.as_str()
                ));
            }
        }
    }

    Ok(())
}

fn validate_subject_or_predicate_term(term: &NqlTerm, position: &str) -> NosqoResult<()> {
    if matches!(term, NqlTerm::Value(_)) {
        return Err(err!("{position} terms must be variables or identifiers"));
    }

    Ok(())
}

fn bound_variables(query: &NqlQuery) -> Vec<NqlVariable> {
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

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::validate_nql_query;

    #[test]
    fn rejects_literal_subjects() {
        let query =
            nosqo_parser::NqlParser::parse_str("match\n\"Berlin\" ~label ?label\nreturn\n?label\n")
                .expect("query should parse");

        let error = validate_nql_query(&query).expect_err("query should be rejected");

        expect![[r#"subject terms must be variables or identifiers"#]]
            .assert_eq(&error.kind().to_string());
    }

    #[test]
    fn rejects_unbound_return_variables() {
        let query = nosqo_parser::NqlParser::parse_str(
            "match\n?city ~label \"Berlin\"\nreturn\n?country\n",
        )
        .expect("query should parse");

        let error = validate_nql_query(&query).expect_err("query should be rejected");

        expect![[r#"return variable `?country` does not appear in the match block"#]]
            .assert_eq(&error.kind().to_string());
    }
}
