//! Events received from Slack through the Events API

use crate::slack;

/// Link
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    /// Registered domain which has been matched
    domain: String,
    /// Unfurled URL
    url: String,
}

/// Informations about an event
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EventInfo {
    /// Subscribe to only the message events that mention your app or bot
    #[serde(rename = "app_mention")]
    AppMention {
        /// User that triggered the event
        user: String,
        /// Text of the message
        text: String,
        /// Timestamp of the message
        ts: String,
        /// Channel of the message
        channel: String,
        /// Timestamp of the event
        event_ts: String,
    },
    /// Your Slack app was uninstalled.
    #[serde(rename = "app_uninstalled")]
    AppUninstalled,
    /// A channel was archived
    #[serde(rename = "channel_archive")]
    ChannelArchive {
        /// Channel that was archived
        channel: String,
        /// User that triggered the event
        user: String,
    },
    /// A channel was created
    #[serde(rename = "channel_created")]
    ChannelCreated {
        /// Channel that was created
        channel: slack::Channel,
    },
    /// A channel was deleted
    #[serde(rename = "channel_deleted")]
    ChannelDeleted {
        /// Channel that was deleted
        channel: String,
    },
    /// Bulk updates were made to a channel's history
    #[serde(rename = "channel_history_changed")]
    ChannelHistoryChanged {
        /// Timestamp
        latest: String,
        /// Timestamp
        ts: String,
        /// Timestamp
        event_ts: String,
    },
    /// You left a channel
    #[serde(rename = "channel_left")]
    ChannelLeft {
        /// Channel that was left
        channel: String,
    },
    /// A channel was renamed
    #[serde(rename = "channel_renamed")]
    ChannelRenamed {
        /// Channel that was renamed
        channel: slack::Channel,
    },
    /// A channel was unarchived
    #[serde(rename = "channel_unarchive")]
    ChannelUnarchive {
        /// Channel that was archived
        channel: String,
        /// User that triggered the event
        user: String,
    },

    /// One or more links have been unfurled
    #[serde(rename = "link_shared")]
    LinkShared {
        /// Channel source of the link(s)
        channel: String,
        /// User that triggered the event
        user: String,
        /// Time stamp of the message
        message_ts: String,
        /// Time stamp of the thread
        thread_ts: String,
        /// Vector of unfurled links
        links: Vec<Link>,
    },
}

/// Event received from Slack
#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Event {
    /// Indicates your app's event subscriptions are being rate limited
    #[serde(rename = "app_rate_limited")]
    AppRateLimited {
        /// The shared-private callback token that authenticates this callback to the application as having come from Slack
        token: String,
        /// The unique identifier for the workspace/team where this event occurred
        team_id: String,
        /// A rounded epoch time value indicating the minute your application became rate limited for this workspace
        minute_rate_limited: u64,
        /// Your application's ID
        api_app_id: String,
    },
    /// The "outer event", or the object containing the event that happened itself.
    #[serde(rename = "event_callback")]
    EventCallback {
        /// The shared-private callback token that authenticates this callback to the application as having come from Slack
        token: String,
        /// The unique identifier for the workspace/team where this event occurred
        team_id: String,
        /// The unique identifier for the application this event is intended for
        api_app_id: String,
        /// Contains the inner set of fields representing the event that's happening
        event: EventInfo,
        /// An array of string-based User IDs. Each member of the collection represents a user that has installed your application/bot and indicates the described event would be visible to those users
        authed_users: Vec<String>,
        /// A unique identifier for this specific event, globally unique across all workspaces
        event_id: String,
        /// The epoch timestamp in seconds indicating when this event was dispatched
        event_time: u64,
    },
    /// Verifies ownership of an Events API Request URL
    #[serde(rename = "url_verification")]
    UrlVerification {
        /// The shared-private callback token that authenticates this callback to the application as having come from Slack
        token: String,
        /// Challenge to reply for the endpoint verification
        challenge: String,
    },
}
