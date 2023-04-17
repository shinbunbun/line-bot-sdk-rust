use crate::awc_wrapper::SendClientRequestByteFut;
use crate::client::API_DATA_ENDPOINT_BASE;

use crate::Client;

impl Client {
    pub fn get_message_content(&self, message_id: &str) -> SendClientRequestByteFut {
        let url = format!(
            "{}/v2/bot/message/{}/content",
            API_DATA_ENDPOINT_BASE, message_id
        );
        SendClientRequestByteFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }
}
