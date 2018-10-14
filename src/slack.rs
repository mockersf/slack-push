//! Types related to Slack objects

/// A Channel in Slack
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    /// Channel ID
    id: String,
    /// Channel Name
    name: String,
    /// Channel creation's timestamp
    created: u64,
    /// Channel creator
    creator: Option<String>,
}
