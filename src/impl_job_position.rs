use async_trait::async_trait;
use serde_json::json;
use tracing::debug;

use super::{Client, JobPositioner, Result, BASE_URL};

#[async_trait]
impl JobPositioner for Client {
    async fn search_position_by_ids(&self, ids: Vec<&str>) -> Result<()> {
        debug!("search_position_by_ids");
        let url = format!(
            "{}/tenantbase/v1/{}/position/ids/search",
            BASE_URL, self.tenant_id
        );

        let post_body = json!({
            "Ids": ids,
            "Columns": vec![
                "Code",            // 编码
                "Name",            // 名称
                "Name_en_US",      // 英文名称
                "Description",     // 职位描述
                "OId",             // OId
                "OIdJobPost",      // 对应职务
                "OIdJobLevelType", // 职级类别
                "OIdJobGrade",     // 职等
                "Status",          // 状态
                "JobRequirements", // 任职要求
                "StdIsDeleted",    // 删除状态
            ],
        });

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                debug!("==>>403");
                if err.is_authentication_error() {
                    return self.search_position_by_ids(ids).await;
                }
                return Err(err);
            }
        };
        // TODO: 暂时没有获取到数据
        let resp: serde_json::Value = resp.json().await?;
        debug!("{}", resp.to_string());
        // Ok(resp.json().await?)
        Ok(())
    }
}
