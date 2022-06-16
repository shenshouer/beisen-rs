use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{interface::Tokener, utils::http::do_http, PostParameters, Result};

use super::Token;

#[derive(Debug)]
pub struct Client {
    pub(crate) app_id: u32,
    pub(crate) secret: String,
    pub(crate) tenant_id: u32,
    pub(crate) token: Arc<Mutex<Token>>,
}

impl Client {
    // 构建客户端实体
    pub fn new(app_id: u32, secret: String, tenant_id: u32) -> Self {
        Client {
            app_id,
            secret,
            tenant_id,
            token: Arc::new(Mutex::new(Token::default())),
        }
    }

    // http 请求
    pub async fn request(&self, url: &str, body: serde_json::Value) -> Result<reqwest::Response> {
        let token = self.access_token().await?;
        let token = format!("Bearer {}", token.access_token);
        #[allow(clippy::mutable_key_type)]
        let headers = std::collections::HashMap::from([(
            reqwest::header::HeaderName::from_str("Authorization")?,
            token,
        )]);

        let resp = do_http(
            reqwest::Method::POST,
            url,
            Some(headers),
            None,
            Some(PostParameters::json(body)),
        )
        .await?;
        Ok(resp)
    }
}
