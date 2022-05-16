use serde::{Deserialize, Serialize};

use super::{quick_reply::QuickReply, sender::Sender, CommonFields};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub base_url: String,
    pub alt_text: String,
    pub base_size: BaseSize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl CommonFields for ImagemapMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl ImagemapMessage {
    pub fn new(base_url: String, alt_text: String, base_size: BaseSize) -> Self {
        ImagemapMessage {
            type_field: "imagemap".to_string(),
            base_url,
            alt_text,
            base_size,
            video: None,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn with_video(mut self, video: Video) -> Self {
        self.video = Some(video);
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSize {
    pub width: u64,
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    original_content_url: String,
    preview_image_url: String,
    area: Area,
    external_link: Option<ExternalLink>,
    actions: Actions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLink {
    link_uri: String,
    label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Actions {
    URIAction(URIAction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct URIAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    pub link_uri: String,
    pub area: Area,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    pub text: String,
    pub area: Area,
}
