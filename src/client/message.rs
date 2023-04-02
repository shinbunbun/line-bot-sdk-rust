use serde::{Deserialize, Serialize};

use crate::{
    awc_wrapper::SendClientRequestFut,
    models::{empty::Empty, message::MessageObject},
    Client,
};

use super::API_ENDPOINT_BASE;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplyRequest {
    reply_token: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushRequest {
    to: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_aggregation_units: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MulticastRequest {
    to: Vec<String>,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_aggregation_units: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BroadcastRequest {
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaResponse {
    #[serde(rename = "type")]
    pub quota_type: String,
    pub value: Option<String>,
}

impl Client {
    pub fn reply(
        &self,
        reply_token: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> SendClientRequestFut<Empty> {
        let body = ReplyRequest {
            reply_token: reply_token.to_string(),
            messages,
            notification_disabled,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/reply", API_ENDPOINT_BASE),
            None,
        ))
    }

    pub fn push(
        &self,
        x_line_retry_key: Option<&str>,
        to: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
        custom_aggregation_units: Option<Vec<String>>,
    ) -> SendClientRequestFut<Empty> {
        let body = PushRequest {
            to: to.to_string(),
            messages,
            notification_disabled,
            custom_aggregation_units,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/push", API_ENDPOINT_BASE),
            x_line_retry_key,
        ))
    }

    pub fn multicast(
        &self,
        x_line_retry_key: Option<&str>,
        to: Vec<String>,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
        custom_aggregation_units: Option<Vec<String>>,
    ) -> SendClientRequestFut<Empty> {
        let body = MulticastRequest {
            to,
            messages,
            notification_disabled,
            custom_aggregation_units,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/multicast", API_ENDPOINT_BASE),
            x_line_retry_key,
        ))
    }

    pub fn broadcast(
        &self,
        x_line_retry_key: Option<&str>,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> SendClientRequestFut<Empty> {
        let body = BroadcastRequest {
            messages,
            notification_disabled,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/broadcast", API_ENDPOINT_BASE),
            x_line_retry_key,
        ))
    }

    pub fn quota(&self) -> SendClientRequestFut<QuotaResponse> {
        SendClientRequestFut::new(self.get::<QuotaResponse>(
            &format!("{}/v2/bot/message/quota", API_ENDPOINT_BASE),
            None,
            None,
            true,
        ))
    }
}