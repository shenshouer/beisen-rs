pub enum TimeWindowOptionKind {
    JobPost,
    Organization,
}

impl TimeWindowOptionKind {
    pub fn get_request_columns(&self) -> &'static [&'static str] {
        match self {
            TimeWindowOptionKind::JobPost => REQUEST_JOBPOST_COLUMNS,
            TimeWindowOptionKind::Organization => REQUEST_ORGANIZATION_COLUMNS,
        }
    }
}

pub(crate) const REQUEST_JOBPOST_COLUMNS: &[&str] = &[
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
];

pub(crate) const REQUEST_ORGANIZATION_COLUMNS: &[&str] = &[
    "Code",                        // 编码	Code	string	✔️	✔️
    "StdIsDeleted",                // 删除状态	StdIsDeleted	boolean	❌	✔️	此字段为系统自动创建，请勿删除
    "BusinessModifiedTime",        // 业务修改时间	BusinessModifiedTime	dateTime	✔️	✔️	最后的业务修改时间
    "BusinessModifiedBy",          // 业务修改人	BusinessModifiedBy	integer	✔️	✔️	最后的业务修改人
    "Name",                        // 名称	Name	string	✔️	✔️
    "Name_zh_TW",                  //名称_繁体	Name_zh_TW	string	✔️	✔️
    "Name_en_US",                  // 名称_英文	Name_en_US	string	✔️	✔️
    "ShortName",                   //简称	ShortName	string	✔️	✔️
    "ShortName_zh_TW",             // 简称_繁体	ShortName_zh_TW	string	✔️	✔️
    "ShortName_en_US",             // 简称_英文	ShortName_en_US	string	✔️	✔️
    "IsVirtualOrg",                // 虚拟组织	IsVirtualOrg	boolean	✔️	✔️
    "LegalMan",                    // 法人代表	LegalMan	string	✔️	✔️
    "Address",                     // 地址	Address	string	✔️	✔️
    "Fax",                         // 传真	Fax	string	✔️	✔️
    "Postcode",                    // 邮编	Postcode	string	✔️	✔️
    "Phone",                       // 电话	Phone	string	✔️	✔️
    "Url",                         // 网址	Url	string	✔️	✔️
    "Description",                 // 简介	Description	string	✔️	✔️
    "Number",                      // 文号	Number	string	✔️	✔️
    "IsDeleted",                   // 是否删除	IsDeleted	boolean	❌	✔️
    "CreatedBy",                   // 创建人	CreatedBy	integer	❌	✔️
    "ModifiedBy",                  // 修改人	ModifiedBy	integer	❌	✔️
    "CreatedTime",                 // 创建时间	CreatedTime	dateTime	❌	✔️
    "ModifiedTime",                // 修改时间	ModifiedTime	dateTime	❌	✔️
    "Level",                       // 层级	Level	string	✔️	✔️
    "BroadType",                   // 大类	BroadType	string	✔️	✔️
    "EconomicType",                // 经济类型	EconomicType	string	✔️	✔️
    "Industry",                    // 所属行业	Industry	string	✔️	✔️
    "Place",                       // 所在地点	Place	string	✔️	✔️
    "Status",                      // 状态	Status	integer	❌	✔️
    "OrderAdmin",                  // 行政维度顺序号	OrderAdmin	integer	✔️	✔️
    "OrderReserve2",               // 业务维度顺序号	OrderReserve2	integer	✔️	✔️
    "OrderReserve3",               // 产品维度顺序号	OrderReserve3	integer	✔️	✔️
    "OrderReserve4",               // 预留维度4顺序号	OrderReserve4	integer	✔️	✔️
    "OrderReserve5",               // 预留维度5顺序号	OrderReserve5	integer	✔️	✔️
    "EstablishDate",               // 设立日期	EstablishDate	dateTime	✔️	✔️
    "StartDate",                   // 生效日期	StartDate	dateTime	❌	✔️
    "StopDate",                    // 失效日期	StopDate	dateTime	❌	✔️
    "ChangeDate",                  // 变更日期	ChangeDate	dateTime	✔️	✔️
    "Attachment",                  // 附件	Attachment	string	✔️	✔️
    "Comment",                     // 备注	Comment	string	✔️	✔️
    "OId",                         // 组织单元Id	OId	integer	❌	✔️
    "OIdOrganizationType",         // 类型	OIdOrganizationType	string	✔️	✔️
    "POIdOrgAdmin",                // 行政维度上级	POIdOrgAdmin	integer	✔️	✔️	行政维度
    "POIdOrgAdmin_TreePath",       // 行政维度_路径	POIdOrgAdmin_TreePath	string	❌	✔️
    "POIdOrgAdmin_TreeLevel",      // 行政维度_层级	POIdOrgAdmin_TreeLevel	integer	❌	✔️
    "POIdOrgReserve2",             // 业务维度上级	POIdOrgReserve2	integer	✔️	✔️	业务维度
    "POIdOrgReserve2_TreePath",    // 业务维度_路径	POIdOrgReserve2_TreePath	string	❌	✔️
    "POIdOrgReserve2_TreeLevel",   // 业务维度_层级	POIdOrgReserve2_TreeLevel	integer	❌	✔️
    "POIdOrgReserve3",             // 产品维度	POIdOrgReserve3	integer	✔️	✔️	产品维度
    "POIdOrgReserve3_TreePath",    // 产品维度_路径	POIdOrgReserve3_TreePath	string	❌	✔️
    "POIdOrgReserve3_TreeLevel",   // 产品维度_层级	POIdOrgReserve3_TreeLevel	integer	❌	✔️
    "POIdOrgReserve4",             // 预留维度4	POIdOrgReserve4	integer	✔️	✔️	预留4维度
    "POIdOrgReserve4_TreePath",    // 预留维度4_路径	POIdOrgReserve4_TreePath	string	❌	✔️
    "POIdOrgReserve4_TreeLevel",   // 预留维度4_层级	POIdOrgReserve4_TreeLevel	integer	❌	✔️
    "POIdOrgReserve5",             // 预留维度5	POIdOrgReserve5	integer	✔️	✔️	预留5维度
    "POIdOrgReserve5_TreePath",    // 预留维度5_路径	POIdOrgReserve5_TreePath	string	❌	✔️
    "POIdOrgReserve5_TreeLevel",   // 预留维度5_层级	POIdOrgReserve5_TreeLevel	integer	❌	✔️
    "_Name_zh_TW",                 // 名称（不分词）_繁体	_Name_zh_TW	string	✔️	✔️
    "_Name_en_US",                 // 名称（不分词）_英文	_Name_en_US	string	✔️	✔️
    "PersonInCharge",              // 负责人	PersonInCharge	integer	✔️	✔️
    "HRBP",                        // HRBP	HRBP	integer	✔️	✔️
    "FirstLevelOrg",               // 一级组织	FirstLevelOrg	long	✔️	✔️
    "SecondLevelOrg",              // 二级组织	SecondLevelOrg	long	✔️	✔️
    "ThirdLevelOrg",               // 三级组织	ThirdLevelOrg	long	✔️	✔️
    "FourthLevelOrg",              // 四级组织	FourthLevelOrg	long	✔️	✔️
    "FifthLevelOrg",               // 五级组织	FifthLevelOrg	long	✔️	✔️
    "SixthLevelOrg",               // 六级组织	SixthLevelOrg	long	✔️	✔️
    "SeventhLevelOrg",             // 七级组织	SeventhLevelOrg	long	✔️	✔️
    "EighthLevelOrg",              // 八级组织	EighthLevelOrg	long	✔️	✔️
    "NinthLevelOrg",               // 九级组织	NinthLevelOrg	long	✔️	✔️
    "TenthLevelOrg",               // 十级组织	TenthLevelOrg	long	✔️	✔️
    "FirstLevelOrganization",      // 一级组织	FirstLevelOrganization	integer	❌	✔️
    "SecondLevelOrganization",     // 二级组织	SecondLevelOrganization	integer	❌	✔️
    "ThirdLevelOrganization",      // 三级组织	ThirdLevelOrganization	integer	❌	✔️
    "FourthLevelOrganization",     // 四级组织	FourthLevelOrganization	integer	❌	✔️
    "FifthLevelOrganization",      // 五级组织	FifthLevelOrganization	integer	❌	✔️
    "SixthLevelOrganization",      // 六级组织	SixthLevelOrganization	integer	❌	✔️
    "SeventhLevelOrganization",    // 七级组织	SeventhLevelOrganization	integer	❌	✔️
    "EighthLevelOrganization",     // 八级组织	EighthLevelOrganization	integer	❌	✔️
    "NinthLevelOrganization",      // 九级组织	NinthLevelOrganization	integer	❌	✔️
    "TenthLevelOrganization",      // 十级组织	TenthLevelOrganization	integer	❌	✔️
    "OrderCode", // 排序编码	OrderCode	long	❌	✔️	用于添加排序编码,该编码是通过配置排序规则,并使用排序编码计算引擎计算出来的一组数值
    "POIdOrgAdminNameTreePath", // 组织全称	POIdOrgAdminNameTreePath	string	❌	✔️	行政维度汇报关系文本翻译
    "ShopOwner", // 店长	ShopOwner	integer	✔️	✔️	在职员工数据源、数据源无MOU权限控制表单中默认不显示
    "AdministrativeAssistant", // 行政助理	AdministrativeAssistant	integer	✔️	✔️
    "IsAddEmploymentRecord", // 是否新增任职	IsAddEmploymentRecord	string	✔️	✔️
    "PersonInChargeDeputy", // 负责人（副职）	PersonInChargeDeputy	string	✔️	✔️	仅作为组织的信息记录，暂不支持负责人（副职）角色人员对组织团队的相关业务处理
    "CostCenterOId",        // 成本中心	CostCenterOId	integer	✔️	✔️
    "WithLowerCount",       // 在岗人数	WithLowerCount	integer	✔️	✔️	在岗人数（含下级）
    "WithLowerVacancyCount", // 缺编数（i62产品确认删除，计算影响性能）	WithLowerVacancyCount	integer	✔️	✔️	缺编数（含下级）
    "WithLowerEstablishment", // 编制数	WithLowerEstablishment	integer	✔️	✔️	编制数（含下级）
    "LeadersWithSpecificDuties", // 分管领导（废弃）	LeadersWithSpecificDuties	string	✔️	✔️
    "LeadersWithSpecificDutiesV2", // 分管领导（废弃）	LeadersWithSpecificDutiesV2	string	✔️	✔️
    "LeaderWithSpecificDuty", // 分管领导	LeaderWithSpecificDuty	integer	✔️	✔️
];

use crate::utils::datetime::date_format;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchTimeWindowOption {
    #[serde(rename(serialize = "StartTime"), with = "date_format")]
    start_time: DateTime<Utc>,
    #[serde(rename(serialize = "StopTime"), with = "date_format")]
    stop_time: DateTime<Utc>,
    #[serde(rename(serialize = "WithDisabled"))]
    with_disabled: bool,
    #[serde(rename(serialize = "WithDeleted"))]
    with_deleted: bool,
    #[serde(rename(serialize = "PageIndex"))]
    page_index: usize,
    #[serde(rename(serialize = "PageSize"))]
    page_size: usize,
    #[serde(rename(serialize = "Columns"))]
    columns: &'static [&'static str],
}

impl SearchTimeWindowOption {
    pub fn new(
        kind: TimeWindowOptionKind,
        start_time: DateTime<Utc>,
        stop_time: DateTime<Utc>,
        page_index: usize,
    ) -> Self {
        Self {
            start_time,
            stop_time,
            with_disabled: true,
            with_deleted: true,
            page_index,
            page_size: 300,
            columns: kind.get_request_columns(),
        }
    }

    // 距离当前N分钟参数
    pub fn new_with_minutes(kind: TimeWindowOptionKind, page_index: usize, minutes: i64) -> Self {
        let stop_time = Utc::now();
        let start_time = stop_time - Duration::minutes(minutes);
        SearchTimeWindowOption::new(kind, start_time, stop_time, page_index)
    }

    // 距离当前N天参数
    pub fn new_with_days(kind: TimeWindowOptionKind, page_index: usize, days: i64) -> Self {
        let stop_time = Utc::now();
        let start_time = stop_time - Duration::days(days);
        tracing::debug!("start_time: {:?} stop_time: {:?}", start_time, stop_time);
        SearchTimeWindowOption::new(kind, start_time, stop_time, page_index)
    }

    pub fn clone_with_page_index(&self, page_index: usize) -> Self {
        Self {
            page_index,
            start_time: self.start_time,
            stop_time: self.stop_time,
            with_disabled: self.with_disabled,
            with_deleted: self.with_deleted,
            page_size: self.page_size,
            columns: REQUEST_ORGANIZATION_COLUMNS,
        }
    }

    pub fn total(&self) -> usize {
        self.page_index * self.page_size
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }
}
