use self::{
    audio::AudioMessage, image::ImageMessage, imagemap::ImagemapMessage, location::LocationMessage,
    quick_reply::QuickReply, sender::Sender, stamp::StampMessage, text::TextMessage,
    video::VideoMessage,
};
use serde::{Deserialize, Serialize};

pub mod audio;
pub mod image;
pub mod imagemap;
pub mod location;
pub mod quick_reply;
pub mod sender;
pub mod stamp;
pub mod template;
pub mod text;
pub mod video;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
    #[serde(flatten)]
    pub message: EachMessageFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EachMessageFields {
    Text(TextMessage),
    Stamp(StampMessage),
    Image(ImageMessage),
    Video(VideoMessage),
    Audio(AudioMessage),
    Location(LocationMessage),
    Imagemap(ImagemapMessage),
}

/* fn test() {
    let s = MessageObject {
        quick_reply: None,
        sender: None,
        message: EachMessageFields::Text(Text {
            text: "hello".to_string(),
            type_field: "text".to_string(),
            emojis: None,
        }),
    };
    let json = serde_json::to_string(&s).unwrap();
    println!("{}", json);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        test();
    }
} */
