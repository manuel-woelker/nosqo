use serde::{Deserialize, Serialize};

use super::NqlVariable;

/// The return specification of an NQL query.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NqlReturn {
    /// Return all bound variables.
    All,
    /// Return the listed variables in order.
    Variables(Vec<NqlVariable>),
}
