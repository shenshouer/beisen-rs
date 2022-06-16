use async_trait::async_trait;

use super::{
    dto::{ObjectName, SearchEmployeeTimeWindowOption, SearchTimeWindowOption},
    model::{contract::Contract, setting::Setting, JobPost},
    Employee, EmployeeBasicInfo, EmployeeServiceInfo, Organization, Result, Token,
};

/// token 接口实现
#[async_trait]
pub trait Tokener {
    /// 获取token
    async fn access_token(&self) -> Result<Token>;
}

/// 劳动合同数据相关接口
#[async_trait]
/// TODO: 暂未实现
pub trait Contracter {
    // 根据员工ID获取员工的所有非删除的合同协议
    async fn search_contract_by_uids(&self, uids: Vec<u32>) -> Result<Vec<Contract>>;
}

/// 员工相关(个人信息/任职信息)数据处理
#[async_trait]
pub trait Employeer: Tokener {
    /// 通过用户邮箱获取用户UserID
    async fn get_uid_by_email(&self, email: &str) -> Result<u32>;
    /// 根据Id集合获取员工基本信息
    async fn get_basicinfo_by_uids(&self, uids: Vec<u32>) -> Result<Vec<EmployeeBasicInfo>>;
    /// 根据Id集合获取员工任职信息
    async fn get_serviceinfo_by_uids(&self, uids: Vec<u32>) -> Result<Vec<EmployeeServiceInfo>>;
    // 通过时间窗方式获取指定范围内发生变化的员工数据
    async fn search_employee_with_timewindow(
        &self,
        opt: &SearchEmployeeTimeWindowOption,
    ) -> Result<Vec<Employee>>;
    // 编辑员工邮箱
    async fn update_email(&self, uid: u32, email: &str) -> Result<()>;
}

/// 员工子集信息相关接口
/// TODO: 暂未实现
#[async_trait]
pub trait EmployeeSubeter {}

/// 职位相关接口
#[async_trait]
pub trait JobPositioner {
    // 根据Id集合获取职位数据
    async fn search_position_by_ids(&self, ids: Vec<&str>) -> Result<()>;
}

/// 职务相关接口
#[async_trait]
pub trait JobPoster {
    // /v1/{tenantId}/jobpost/ids/search
    // 根据Id集合获取职务数据
    async fn search_job_post_by_ids(&self, uids: Vec<u32>) -> Result<()>;
    // /v1/{tenantId}/jobpost/timewindow/search
    // 通过时间窗方式获取指定范围内发生变化的职务数据
    async fn search_job_post_with_timewindow(
        &self,
        opt: &SearchTimeWindowOption,
    ) -> Result<Vec<JobPost>>;
}

/// 职务序列数据相关接口
/// TODO: 暂未实现
#[async_trait]
pub trait JobSequeencer {}

/// 元数据、数据源相关接口
/// TODO: 暂未实现
#[async_trait]
pub trait Metaer {}

/// 组织相关接口
#[async_trait]
pub trait Organizationer {
    // 根据Id集合获取组织数据
    async fn search_organization_by_ids(&self, oids: Vec<u32>) -> Result<Vec<Organization>>;
    // 通过时间窗方式获取指定范围内发生变化的组织数据
    async fn search_organization_with_timewindow(
        &self,
        opt: &SearchTimeWindowOption,
    ) -> Result<Vec<Organization>>;
}

/// 设置类对象数据
#[async_trait]
pub trait Settinger {
    // 分页获取JobLevel对象
    async fn list_setting(&self, name: ObjectName) -> Result<Setting>;
    // 根据Id集合获取指定设置类对象的数据
    async fn search_setting_by_ids(&self, name: ObjectName, ids: Vec<&str>) -> Result<Setting>;
}
