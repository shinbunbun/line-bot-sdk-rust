use serde::Deserialize;
use serde::Serialize;

use crate::awc_wrapper::SendClientRequestFut;
use crate::models::empty::Empty;
use crate::Client;

use super::API_ENDPOINT_BASE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenResponse {
    pub client_id: String,
    pub expires_in: i64,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenV2Response {
    pub client_id: String,
    pub expires_in: i64,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokensKidResponse {
    pub kids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueTokenV2Response {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

impl Client {
    pub fn issue_token(&self, client_assertion: &str) -> SendClientRequestFut<IssueTokenResponse> {
        SendClientRequestFut::new(self.post_form(
            [
                ("grant_type", "client_credentials"),
                (
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                ),
                ("client_assertion", client_assertion),
            ],
            &format!("{}/oauth2/v2.1/token", API_ENDPOINT_BASE),
        ))
    }

    pub fn verify_token(&self, access_token: &str) -> SendClientRequestFut<VerifyTokenResponse> {
        SendClientRequestFut::new(self.get(
            &format!("{}/oauth2/v2.1/verify", API_ENDPOINT_BASE),
            Some(&[("access_token", access_token)]),
            Some("application/x-www-form-urlencoded"),
            false,
        ))
    }

    pub fn get_tokens_kid(
        &self,
        client_assertion: &str,
    ) -> SendClientRequestFut<GetTokensKidResponse> {
        SendClientRequestFut::new(self.get(
            &format!("{}/oauth2/v2.1/tokens/kid", API_ENDPOINT_BASE),
            Some(&[
                (
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                ),
                ("client_assertion", client_assertion),
            ]),
            Some("application/x-www-form-urlencoded"),
            false,
        ))
    }

    pub fn revoke_token(
        &self,
        client_id: &str,
        client_secret: &str,
        access_token: &str,
    ) -> SendClientRequestFut<Empty> {
        SendClientRequestFut::new(self.post_form(
            [
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("access_token", access_token),
            ],
            &format!("{}/oauth2/v2.1/revoke", API_ENDPOINT_BASE),
        ))
    }

    pub fn issue_token_v2(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> SendClientRequestFut<IssueTokenV2Response> {
        SendClientRequestFut::new(self.post_form(
            [
                ("grant_type", "client_credentials"),
                ("client_id", client_id),
                ("client_secret", client_secret),
            ],
            &format!("{}/v2/oauth/accessToken", API_ENDPOINT_BASE),
        ))
    }

    pub fn verify_token_v2(
        &self,
        access_token: &str,
    ) -> SendClientRequestFut<VerifyTokenV2Response> {
        SendClientRequestFut::new(self.post_form(
            [("access_token", access_token)],
            &format!("{}/v2/oauth/verify", API_ENDPOINT_BASE),
        ))
    }

    pub fn revoke_token_v2(&self, access_token: &str) -> SendClientRequestFut<Empty> {
        SendClientRequestFut::new(self.post_form(
            [("access_token", access_token)],
            &format!("{}/v2/oauth/revoke", API_ENDPOINT_BASE),
        ))
    }
}
