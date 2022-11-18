use async_recursion::async_recursion;
use async_trait::async_trait;
use serde_json::{json, Value};
use tracing::debug;

use super::{
    dto::ObjectName,
    model::{
        response::BeisenResponse,
        setting::{JobLevel, Setting},
    },
    Client, Result, Settinger, BASE_URL,
};

impl ObjectName {
    // 根据ObjectName获取对应返回类型数据
    pub fn response(&self, value: Value) -> Result<Setting> {
        match self {
            ObjectName::JobLevel => Ok(Setting::JobLevels(serde_json::from_value(value)?)),
        }
    }
}

impl Client {
    // 分页获取JobLevel对象
    #[async_recursion]
    async fn list_setting_by_page(&self, name: ObjectName, page: usize) -> Result<Vec<JobLevel>> {
        debug!("list_setting_by_page");
        let url = format!(
            "{}/tenantbase/v1/{}/setting/{}/page",
            BASE_URL,
            self.tenant_id,
            name.get_name()
        );

        let post_body = json!({
            "PageIndex": page,
            "PageSize":  300,
            "Columns": name.get_request_columns(),
        });

        let resp = match self.request(None, &url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.list_setting_by_page(name, page).await;
                }
                return Err(err);
            }
        };

        let mut items = vec![];
        let resp: BeisenResponse<Vec<JobLevel>, u32> = resp.json().await?;
        if let Some(mut data) = resp.data {
            items.append(&mut data);
        }

        debug!("total: {}, current: {}", resp.total, page * 300);

        if page * 300 < resp.total {
            let next_page = page + 1;
            debug!("next_page_index: {}", next_page);

            let mut datas = self.list_setting_by_page(name, next_page).await?;
            items.append(&mut datas);
        }
        Ok(items)
    }
}

#[async_trait]
impl Settinger for Client {
    async fn list_setting(&self, name: ObjectName) -> Result<Setting> {
        let items = self.list_setting_by_page(name, 1).await?;
        name.response(serde_json::to_value(items)?)
    }

    // 根据Id集合获取指定设置类对象的数据
    async fn search_setting_by_ids(&self, name: ObjectName, ids: Vec<&str>) -> Result<Setting> {
        debug!("search_setting_by_ids");
        let url = format!(
            "{}/tenantbase/v1/{}/setting/{}/ids/search",
            BASE_URL,
            self.tenant_id,
            name.get_name()
        );

        let post_body = json!({
            "Ids": ids,
            "Columns": name.get_request_columns(),
        });

        let resp = match self.request(None, &url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.search_setting_by_ids(name, ids).await;
                }
                return Err(err);
            }
        };
        name.response(resp.json().await?)
    }
}
