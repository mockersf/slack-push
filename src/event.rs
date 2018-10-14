use slack;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EventCallback {
    #[serde(rename = "app_mention")]
    AppMention {
        user: String,
        text: String,
        ts: String,
        channel: String,
        event_ts: String,
    },
    #[serde(rename = "app_uninstalled")]
    AppUninstalled,
    #[serde(rename = "channel_archive")]
    ChannelArchive { channel: String, user: String },
    #[serde(rename = "channel_created")]
    ChannelCreated { channel: slack::Channel },
    #[serde(rename = "channel_deleted")]
    ChannelDeleted { channel: String },
    #[serde(rename = "channel_history_changed")]
    ChannelHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    #[serde(rename = "channel_left")]
    ChannelLeft { channel: String },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed { channel: slack::Channel },
    #[serde(rename = "channel_unarchive")]
    ChannelUnarchive { channel: String, user: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "url_verification")]
    UrlVerification { token: String, challenge: String },
    #[serde(rename = "app_rate_limited")]
    AppRateLimited {
        token: String,
        team_id: String,
        minute_rate_limited: u64,
        api_app_id: String,
    },
    #[serde(rename = "event_callback")]
    EventCallback {
        token: String,
        team_id: String,
        api_app_id: String,
        event: EventCallback,
        event_id: String,
        event_time: u64,
    },
}
