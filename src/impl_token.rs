use async_trait::async_trait;
use serde_json::json;

use super::{
    model::Token, utils::http::do_http, Client, PostParameters, Result, Tokener, BASE_URL,
};

#[async_trait]
impl Tokener for Client {
    async fn access_token(&self) -> Result<Token> {
        let mut token = self.token.lock().await;
        if !token.is_expires() {
            return Ok(token.clone());
        }

        let post_body = json!({
            "app_id": self.app_id,
            "secret": self.secret,
            "tenant_id": self.tenant_id,
            "grant_type": "client_credentials",
        });

        let resp = do_http(
            reqwest::Method::POST,
            &format!("{}/OAuth/Token", BASE_URL),
            None,
            None,
            Some(PostParameters::form(post_body)),
        )
        .await?;

        let data: Token = resp.json().await?;

        token.clone_from(&data);
        Ok(data)
    }
}
