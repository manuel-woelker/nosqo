use nosqo_base::result::NosqoResult;
use nosqo_model::{StatementPattern, StatementSet};

/// Storage abstraction for asserting and querying statements.
pub trait StatementStore {
    /// Asserts that the provided statements exist in the store.
    fn assert_statements(&self, statement_set: StatementSet) -> NosqoResult<()>;

    /// Finds all statements matching the provided pattern.
    fn find_statements(&self, pattern: &StatementPattern) -> NosqoResult<StatementSet>;
}
