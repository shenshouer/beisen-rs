use async_trait::async_trait;
use serde_json::json;
use tracing::debug;

use super::{
    dto::{SearchEmployeeTimeWindowOption, REQUEST_EMPLOYEE_COLUMNS},
    interface::Employeer,
    model::response::BeisenResponse,
    Client, Employee, EmployeeBasicInfo, EmployeeServiceInfo, Result, BASE_URL,
};

#[async_trait]
impl Employeer for Client {
    #[tracing::instrument]
    async fn get_uid_by_email(&self, email: &str) -> Result<u32> {
        let url = format!(
            "{}/tenantbase/v1/{}/employee/seviceinfo/email/search",
            BASE_URL, self.tenant_id
        );

        let post_body = json!({
            "Email":   email,
            "Columns": REQUEST_EMPLOYEE_COLUMNS,
        });

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.get_uid_by_email(email).await;
                }
                return Err(err);
            }
        };
        let resp: BeisenResponse<String, String> = resp.json().await?;

        // Ok(resp.data.parse()?)
        if let Some(data) = resp.data {
            return Ok(data.parse()?);
        }
        Ok(0)
    }

    async fn get_basicinfo_by_uids(&self, uids: Vec<u32>) -> Result<Vec<EmployeeBasicInfo>> {
        let url = format!(
            "{}/tenantbase/v1/{}/employee/basicinfo/ids/search",
            BASE_URL, self.tenant_id
        );

        let post_body = json!({
            "Ids":   uids,
            "Columns": REQUEST_EMPLOYEE_COLUMNS,
        });

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.get_basicinfo_by_uids(uids).await;
                }
                return Err(err);
            }
        };

        Ok(resp.json().await?)
    }

    async fn get_serviceinfo_by_uids(&self, uids: Vec<u32>) -> Result<Vec<EmployeeServiceInfo>> {
        let url = format!(
            "{}/tenantbase/v1/{}/employee/serviceinfo/ids/search",
            BASE_URL, self.tenant_id
        );

        let post_body = json!({
            "Ids":   uids,
            "Columns": REQUEST_EMPLOYEE_COLUMNS,
        });

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.get_serviceinfo_by_uids(uids).await;
                }
                return Err(err);
            }
        };

        Ok(resp.json().await?)
    }

    async fn search_employee_with_timewindow(
        &self,
        opt: &SearchEmployeeTimeWindowOption,
    ) -> Result<Vec<Employee>> {
        let url = format!(
            "{}/tenantbase/v1/{}/employee/timewindow/search",
            BASE_URL, self.tenant_id
        );

        let post_body = serde_json::to_value(opt)?;

        let resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.search_employee_with_timewindow(opt).await;
                }
                return Err(err);
            }
        };

        let mut items = vec![];
        let resp: BeisenResponse<Vec<Employee>, u32> = resp.json().await?;
        if let Some(mut data) = resp.data {
            items.append(&mut data);
        }
        // 如果当前加载的数据还未达到总数, 则说明还有分页数据
        debug!("total: {}, current: {}", resp.total, opt.total());

        if opt.total() < resp.total {
            let next_page_index = opt.page_index() + 1;
            debug!("next_page_index: {}", next_page_index);

            let next_opt = opt.clone_with_page_index(next_page_index);
            let mut datas = self.search_employee_with_timewindow(&next_opt).await?;
            items.append(&mut datas);
        }
        Ok(items)
    }

    async fn update_email(&self, uid: u32, email: &str) -> Result<()> {
        let url = format!(
            "{}/tenantbase/v1/{}/employee/{}",
            BASE_URL, self.tenant_id, uid
        );

        let post_body = json!({
            "BasicInfoFields": {
                "Email": email,
                "WorkEmail": email,
            }
        });

        let _resp = match self.request(&url, post_body).await {
            Ok(t) => t,
            Err(err) => {
                if err.is_authentication_error() {
                    return self.update_email(uid, email).await;
                }
                return Err(err);
            }
        };

        Ok(())
    }
}
