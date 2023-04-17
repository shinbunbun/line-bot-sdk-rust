use serde::{Deserialize, Serialize};

use crate::awc_wrapper::{SendClientRequestByteFut, SendClientRequestFut};
use crate::client::API_DATA_ENDPOINT_BASE;

use crate::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessageContentTranscodingResponse {
    pub status: String,
}

impl Client {
    pub fn get_message_content(&self, message_id: &str) -> SendClientRequestByteFut {
        let url = format!(
            "{}/v2/bot/message/{}/content",
            API_DATA_ENDPOINT_BASE, message_id
        );
        SendClientRequestByteFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }

    pub fn get_message_content_transcoding(
        &self,
        message_id: &str,
    ) -> SendClientRequestFut<GetMessageContentTranscodingResponse> {
        let url = format!(
            "{}/v2/bot/message/{}/content/transcoding",
            API_DATA_ENDPOINT_BASE, message_id
        );
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }
}
