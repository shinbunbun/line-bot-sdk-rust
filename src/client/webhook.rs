use serde::{Deserialize, Serialize};

use crate::awc_wrapper::SendClientRequestFut;
use crate::error::Error;
use crate::Client;

use super::API_ENDPOINT_BASE;

impl Client {
    pub fn set_webhook_event_url(&self, endpoint: &str) -> SendClientRequestFut<()> {
        SendClientRequestFut::new(self.put(
            WebhookEndpointStruct {
                endpoint: endpoint.to_string(),
            },
            &format!("{}/v2/bot/channel/webhook/endpoint", API_ENDPOINT_BASE),
        ))
    }

    pub async fn get_webhook_endpoint_info(&self) -> Result<WebhookEndpointInfo, Error> {
        let mut res = self
            .get(
                &format!("{}/v2/bot/channel/webhook/endpoint", API_ENDPOINT_BASE),
                None::<&[(); 0]>,
                None,
                true,
            )?
            .await
            .map_err(Error::AwcSendRequestError)?;
        let res_body = res
            .body()
            .await
            .map_err(Error::ActixWebPayloadError)?
            .to_vec();
        serde_json::from_slice(&res_body).map_err(Error::SerdeJsonError)
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
                (),
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
