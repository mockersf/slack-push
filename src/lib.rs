extern crate serde;
#[macro_use]
extern crate serde_derive;

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

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct MessageStandard {
    pub attachments: Option<Vec<MessageStandardAttachment>>,
    pub bot_id: Option<String>,
    pub channel: Option<String>,
    pub token: Option<String>,
    pub edited: Option<MessageStandardEdited>,
    pub event_ts: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub source_team: Option<String>,
    pub team: Option<String>,
    pub text: Option<String>,
    pub thread_ts: Option<String>,
    pub ts: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct MessageStandardAttachment {
    pub actions: Option<Vec<MessageStandardAttachmentAction>>,
    pub author_icon: Option<String>,
    pub author_link: Option<String>,
    pub author_name: Option<String>,
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub fields: Option<Vec<MessageStandardAttachmentField>>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub image_url: Option<String>,
    pub pretext: Option<String>,
    pub text: Option<String>,
    pub thumb_url: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub ts: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct MessageStandardAttachmentAction {
    pub url: Option<String>,
    pub text: Option<String>,
    pub style: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct MessageStandardAttachmentField {
    pub short: Option<bool>,
    pub title: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct MessageStandardEdited {
    pub ts: Option<String>,
    pub user: Option<String>,
}
