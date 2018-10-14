//! Types related to Slack messages

/// A message to send
#[derive(Clone, Debug, Default, Serialize)]
pub struct Message {
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name
    pub channel: String,
    /// Text of the message to send
    pub text: String,
    /// Pass true to post the message as the authed user, instead of as a bot
    pub as_user: Option<String>,
    /// An array of structured attachments
    pub attachments: Option<Vec<Attachment>>,
    /// Emoji to use as the icon for this message
    pub icon_emoji: Option<String>,
    /// URL to an image to use as the icon for this message
    pub icon_url: Option<String>,
    /// Find and link channel names and usernames
    pub link_names: Option<bool>,
    /// Enable Slack markup parsing
    pub mrkdwn: Option<String>,
    /// Change how messages are treated
    pub parse: Option<String>,
    /// Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation
    pub reply_broadcast: Option<String>,
    /// Provide another message's `ts` value to make this message a reply
    pub thread_ts: Option<String>,
    /// Pass true to enable unfurling of primarily text-based content
    pub unfurl_links: Option<String>,
    /// Pass false to disable unfurling of media content
    pub unfurl_media: Option<String>,
    /// Set your bot's user name
    pub username: Option<String>,
}

/// Attachments let you add more context to a message, making them more useful and effective
#[derive(Clone, Debug, Default, Serialize)]
pub struct Attachment {
    /// A plain-text summary of the attachment
    pub fallback: Option<String>,
    /// Like traffic signals, color-coding messages can quickly communicate intent and help separate them from the flow of other messages in the timeline. Can either be one of `good`, `warning`, `danger`, or any hex color code
    pub color: Option<String>,
    /// This is optional text that appears above the message attachment block
    pub pretext: Option<String>,
    /// Small text used to display the author's name
    pub author_name: Option<String>,
    /// A valid URL that will hyperlink the `author_name`
    pub author_link: Option<String>,
    /// A valid URL that displays a small 16x16px image to the left of the `author_name` text
    pub author_icon: Option<String>,
    /// Will be displayed as larger, bold text near the top of a message attachment
    pub title: Option<String>,
    /// By passing a valid URL, the `title` text will be hyperlinked
    pub title_link: Option<String>,
    /// This is the main text in a message attachment
    pub text: Option<String>,
    /// Array of `Field`s to display
    pub fields: Option<Vec<AttachmentField>>,
    /// A valid URL to an image file that will be displayed inside a message attachment
    pub image_url: Option<String>,
    /// A valid URL to an image file that will be displayed as a thumbnail on the right side of a message attachment
    pub thumb_url: Option<String>,
    /// Some brief text to help contextualize and identify an attachment
    pub footer: Option<String>,
    /// To render a small icon beside your `footer` text, provide a publicly accessible URL string
    pub footer_icon: Option<String>,
    /// By providing the `ts` field with an integer value in "epoch time", the attachment will display an additional timestamp value as part of the attachment's footer
    pub ts: Option<f32>,
    /// The provided string will act as a unique identifier for the collection of buttons within the attachment. It will be sent back to your message button action URL with each invoked action
    pub callback_id: Option<String>,
    /// `AttachmentAction` displayed
    pub actions: Option<Vec<AttachmentAction>>,
    /// Even for message menus, remains the default value `default`
    pub attachment_type: Option<String>,
}

/// Simplify complex workflows and empower users to take decisive action by adding interactive buttons to your messages
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum AttachmentAction {
    /// Interactive button
    #[serde(rename = "button")]
    Button {
        /// Text of the button
        text: String,
        /// Provide a string to give this specific action a name. The name will be returned to your Action URL along with the message's `callback_id` when this action is invoked
        name: Option<String>,
        /// Provide a string identifying this specific action. It will be sent to your Action URL along with the name and attachment's `callback_id`
        value: Option<String>,
        /// URL to open for a `link button`
        url: Option<String>,
        /// Style of the button, can be `danger`, `primary` or `default`
        style: Option<String>,
        /// Confirmation for the button action
        confirm: Option<AttachmentActionConfirm>,
    },
    /// Interactive menu
    #[serde(rename = "select")]
    Select {
        /// Text of the menu
        text: String,
        /// Provide a string to give this specific action a name. The name will be returned to your Action URL along with the message's `callback_id` when this action is invoked
        name: Option<String>,
        /// Provide a string identifying this specific action. It will be sent to your Action URL along with the name and attachment's `callback_id`
        value: Option<String>,
        /// Confirmation for the button action
        confirm: Option<AttachmentActionConfirm>,
        /// The individual options to appear in this menu
        options: Option<SelectMenuOption>,
        /// An alternate, semi-hierarchal way to list available options
        option_groups: Option<SelectMenuOptionGroup>,
        /// Accepts `static`, `users`, `channels`, `conversations`, or `external`
        data_source: Option<String>,
        /// The first element of this array will be set as the pre-selected option for this menu
        selected_options: Option<SelectMenuOption>,
        /// Only applies when `data_source` is set to `external`. If present, Slack will wait till the specified number of characters are entered before sending a request to your app's external suggestions API endpoint
        min_query_length: Option<u32>,
    },
}

/// An option field
#[derive(Clone, Debug, Default, Serialize)]
pub struct SelectMenuOption {
    /// A short, user-facing string to label this option to users
    pub text: String,
    /// A short string that identifies this particular option to your application
    pub value: String,
    /// A user-facing string that provides more details about this option
    pub description: Option<String>,
}

/// An option group
#[derive(Clone, Debug, Default, Serialize)]
pub struct SelectMenuOptionGroup {
    /// A short, user-facing string to label this option to users
    pub text: String,
    /// The individual options to appear in this message menu
    pub options: Vec<SelectMenuOption>,
}

/// Protect users from destructive actions or particularly distinguished decisions by asking them to confirm their button click one more time
#[derive(Clone, Debug, Default, Serialize)]
pub struct AttachmentActionConfirm {
    /// Title the pop up window
    pub title: Option<String>,
    /// Describe in detail the consequences of the action and contextualize your button text choices
    pub text: Option<String>,
    /// The text label for the button to continue with an action
    pub ok_text: Option<String>,
    /// The text label for the button to cancel the action
    pub dismiss_text: Option<String>,
}

/// `Field`s get displayed in a table-like way
#[derive(Clone, Debug, Default, Serialize)]
pub struct AttachmentField {
    /// Shown as a bold heading above the `value` text
    pub title: Option<String>,
    /// The text value of the field
    pub value: Option<String>,
    /// An optional flag indicating whether the `value` is short enough to be displayed side-by-side with other values
    pub short: Option<bool>,
}
