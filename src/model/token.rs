use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub expires_in: String,
    pub tenant_id: String,
    pub user_id: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            access_token: Default::default(),
            expires_in: "0".to_owned(),
            tenant_id: Default::default(),
            user_id: Default::default(),
        }
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
impl Token {
    // 检查token是否过期
    pub fn is_expires(&self) -> bool {
        let ts = self.expires_in.parse::<u64>().expect("Invalid expires_in");
        if ts == 0 {
            return true;
        }
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        ts - since_the_epoch.as_secs() < 10
    }
}
