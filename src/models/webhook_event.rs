use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub destination: String,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    pub type_field: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub webhook_event_id: String,
    pub delivery_context: DeliveryContext,
    pub message: Option<Message>,
    pub reply_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContext {
    pub is_redelivery: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Text(Text),
    Image(Image),
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    pub emojis: Option<Vec<Emoji>>,
    pub mention: Option<Mention>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content_provider: ContentProvider,
    pub image_set: Option<ImageSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentProvider {
    #[serde(rename = "type")]
    pub type_field: String,
    pub original_content_url: Option<String>,
    pub preview_image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSet {
    pub id: String,
    pub index: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub index: i64,
    pub length: i64,
    pub product_id: String,
    pub emoji_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub mentionees: Vec<Mentionee>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mentionee {
    pub index: i64,
    pub length: i64,
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "type")]
    pub type_field: String,
    pub user_id: String,
    pub group_id: Option<String>,
    pub room_id: Option<String>,
}
