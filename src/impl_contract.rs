use async_trait::async_trait;
use serde_json::json;
use tracing::debug;

use super::{model::contract::Contract, Client, Contracter, Result, BASE_URL};

#[async_trait]
impl Contracter for Client {
    async fn search_contract_by_uids(&self, uids: Vec<u32>) -> Result<Vec<Contract>> {
        debug!("search_contract_by_uids");
        let url = format!(
            "{}/tenantbase/v1/{}/contract/ids/search",
            BASE_URL, self.tenant_id
        );

        let post_body = json!({
            "Ids": uids,
            "Columns": vec![
                "FirstParty",          // 公司名称
                "UserID",              //
                "StdIsDeleted",        // 删除状态
                "IsDeleted",           // 是否删除
                "TerminateDate",       // 终止日期
                "ActualTerminateDate", // 实际终止日期
                "SigningDate",         // 签订日期
            ],
        });

        let resp = match self.request(None, &url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.search_contract_by_uids(uids).await;
                }
                return Err(err);
            }
        };
        // let resp: serde_json::Value = resp.json().await?;
        // debug!("{}", resp.to_string());
        Ok(resp.json().await?)
        // Ok(())
    }
}
