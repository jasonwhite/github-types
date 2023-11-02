use crate::Oid;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ApiCommit {
    pub sha: Oid,
    pub url: String,
}
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Branch {
    pub name: String,
    pub commit: ApiCommit,
    pub protected: bool,
}
