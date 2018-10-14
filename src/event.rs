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
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "url_verification")]
    UrlVerification { token: String, challenge: String },
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
