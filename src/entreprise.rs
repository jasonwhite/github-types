use serde::Deserialize;

use crate::DateTime;

/// Information about an entreprise
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Entreprise {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub slug: String,
    pub avatar_url: String,
    pub description: String,
    pub website_url: String,
    pub html_url: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
