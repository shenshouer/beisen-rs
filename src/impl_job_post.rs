use async_trait::async_trait;
use serde_json::json;
use tracing::debug;

use super::{
    dto::SearchTimeWindowOption, model::JobPost, response::BeisenResponse, Client, JobPoster,
    Result, BASE_URL,
};

#[async_trait]
impl JobPoster for Client {
    //
    // 根据Id集合获取职务数据
    async fn search_job_post_by_ids(&self, ids: Vec<u32>) -> Result<Vec<JobPost>> {
        debug!("search_job_post_by_uids");
        let url = format!(
            "{}/tenantbase/v1/{}/jobpost/ids/search",
            BASE_URL, self.tenant_id
        );

        let post_body = json!({
            "Ids": ids,
            "Columns": vec![
                "OId",             // 对象ID
                "Code",            // 编码
                "Name",            // 名称
                "Name_en_US",      // 英文名称
                "Description",     // 职位描述
                "OId",             // OId
                "OIdJobPost",      // 对应职务
                "OIdJobLevelType", // 职级类别
                "Status",          // 状态
                "JobRequirements", // 任职要求
                "IsDeleted",       // 是否删除
            ],
        });

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                debug!("==>>403");
                if err.is_authentication_error() {
                    return self.search_job_post_by_ids(ids).await;
                }
                return Err(err);
            }
        };
        // let resp: serde_json::Value = resp.json().await?;
        // debug!("{}", resp.to_string());
        Ok(resp.json().await?)
        // Ok(())
    }
    //
    // 通过时间窗方式获取指定范围内发生变化的职务数据
    async fn search_job_post_with_timewindow(
        &self,
        opt: &SearchTimeWindowOption,
    ) -> Result<Vec<JobPost>> {
        let url = format!(
            "{}/tenantbase/v1/{}/jobpost/timewindow/search",
            BASE_URL, self.tenant_id
        );

        let post_body = serde_json::to_value(opt)?;

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.search_job_post_with_timewindow(opt).await;
                }
                return Err(err);
            }
        };

        let mut items = vec![];
        // let resp: serde_json::Value = resp.json().await?;
        // tracing::debug!("{}", resp.to_string());
        let resp: BeisenResponse<Vec<JobPost>, u32> = resp.json().await?;
        if let Some(mut data) = resp.data {
            items.append(&mut data);
        }
        // 如果当前加载的数据还未达到总数, 则说明还有分页数据
        debug!("total: {}, current: {}", resp.total, opt.total());

        if opt.total() < resp.total {
            let next_page_index = opt.page_index() + 1;
            debug!("next_page_index: {}", next_page_index);

            let next_opt = opt.clone_with_page_index(next_page_index);
            let mut datas = self.search_job_post_with_timewindow(&next_opt).await?;
            items.append(&mut datas);
        }
        Ok(items)
        // Ok(())
    }
}
