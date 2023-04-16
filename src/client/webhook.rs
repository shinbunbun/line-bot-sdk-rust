use serde::{Deserialize, Serialize};

use crate::awc_wrapper::SendClientRequestFut;
use crate::models::empty::Empty;
use crate::Client;

use super::API_ENDPOINT_BASE;

impl Client {
    pub fn set_webhook_event_url(&self, endpoint: &str) -> SendClientRequestFut<Empty> {
        SendClientRequestFut::new(self.put(
            WebhookEndpointStruct {
                endpoint: endpoint.to_string(),
            },
            &format!("{}/v2/bot/channel/webhook/endpoint", API_ENDPOINT_BASE),
        ))
    }

    pub fn get_webhook_endpoint_info(&self) -> SendClientRequestFut<WebhookEndpointInfo> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/channel/webhook/endpoint", API_ENDPOINT_BASE),
            None::<&[(); 0]>,
            None,
            true,
        ))
    }

    pub fn test_webhook_endpoint_url(
        &self,
        endpoint: Option<&str>,
    ) -> SendClientRequestFut<TestWebhookEndpointUrlResponse> {
        match endpoint {
            Some(endpoint) => SendClientRequestFut::new(self.post(
                WebhookEndpointStruct {
                    endpoint: endpoint.to_string(),
                },
                &format!("{}/v2/bot/channel/webhook/endpoint/test", API_ENDPOINT_BASE),
                None,
            )),
            None => SendClientRequestFut::new(self.post(
                Empty {},
                &format!("{}/v2/bot/channel/webhook/endpoint/test", API_ENDPOINT_BASE),
                None,
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebhookEndpointStruct {
    endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpointInfo {
    pub endpoint: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestWebhookEndpointUrlResponse {
    pub success: bool,
    pub timestamp: u64,
    pub status_code: i16,
    pub reason: String,
    pub detail: String,
}
