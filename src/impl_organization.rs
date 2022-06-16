use async_trait::async_trait;
use serde_json::json;
use tracing::debug;

use super::{
    dto::{SearchTimeWindowOption, REQUEST_ORGANIZATION_COLUMNS},
    model::response::BeisenResponse,
    Client, Organization, Organizationer, Result, BASE_URL,
};

#[async_trait]
impl Organizationer for Client {
    async fn search_organization_by_ids(&self, oids: Vec<u32>) -> Result<Vec<Organization>> {
        let url = format!(
            "{}/tenantbase/v1/{}/organization/ids/search",
            BASE_URL, self.tenant_id
        );

        let post_body = json!({
            "Ids":   oids,
            "Columns": REQUEST_ORGANIZATION_COLUMNS,
        });

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.search_organization_by_ids(oids).await;
                }
                return Err(err);
            }
        };

        Ok(resp.json().await?)
    }

    async fn search_organization_with_timewindow(
        &self,
        opt: &SearchTimeWindowOption,
    ) -> Result<Vec<Organization>> {
        let url = format!(
            "{}/tenantbase/v1/{}/organization/timewindow/search",
            BASE_URL, self.tenant_id
        );

        let post_body = serde_json::to_value(opt)?;

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.search_organization_with_timewindow(opt).await;
                }
                return Err(err);
            }
        };

        let mut items = vec![];
        // let resp: serde_json::Value = resp.json().await?;
        // tracing::debug!("{}", resp.to_string());
        let resp: BeisenResponse<Vec<Organization>, u32> = resp.json().await?;
        if let Some(mut data) = resp.data {
            items.append(&mut data);
        }
        // 如果当前加载的数据还未达到总数, 则说明还有分页数据
        debug!("total: {}, current: {}", resp.total, opt.total());

        if opt.total() < resp.total {
            let next_page_index = opt.page_index() + 1;
            debug!("next_page_index: {}", next_page_index);

            let next_opt = opt.clone_with_page_index(next_page_index);
            let mut datas = self.search_organization_with_timewindow(&next_opt).await?;
            items.append(&mut datas);
        }
        Ok(items)
    }
}
