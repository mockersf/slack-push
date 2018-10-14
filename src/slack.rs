//! Types related to Slack objects

/// A Channel in Slack
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    /// Channel ID
    pub id: String,
    /// Channel Name
    pub name: String,
    /// Channel creation's timestamp
    pub created: u64,
    /// Channel creator
    pub creator: Option<String>,
}
