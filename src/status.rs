use crate::Oid;
use serde::Deserialize;

#[derive(
    Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum StatusState {
    Error,
    Failure,
    Pending,
    Success,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StatusCommit {
    pub sha: Oid,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
}
