use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
    #[serde(rename = "BasicInfos")]
    basic_info: EmployeeBasicInfo,
    #[serde(rename = "ServiceInfos")]
    service_infos: Vec<EmployeeServiceInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmployeeServiceInfo {
    // 人员	UserID	integer	❌	✔️
    #[serde(rename = "UserID")]
    user_id: u32,
    #[serde(default, rename = "ObjectId")]
    object_id: String,
    #[serde(rename = "BusinessModifiedBy")]
    // 业务修改人	BusinessModifiedBy	integer	✔️	✔️	最后的业务修改人
    business_modified_by: u32,
    #[serde(rename = "BusinessModifiedTime", with = "date_format")]
    // 业务修改时间	BusinessModifiedTime	dateTime	✔️	✔️	最后的业务修改时间
    business_modified_time: DateTime<Utc>,
    // 机构	OIdOrganization	integer	✔️	✔️
    #[serde(rename = "OIdOrganization")]
    organization_id: u32,
    // 删除状态	StdIsDeleted	boolean	❌	✔️	此字段为系统自动创建，请勿删除
    #[serde(rename = "StdIsDeleted")]
    std_is_deleted: bool,
    // 部门	OIdDepartment	integer	✔️	✔️
    #[serde(rename = "OIdDepartment")]
    department_id: u32,
    // 职务	OIdJobPost	integer	✔️	✔️
    #[serde(default, rename = "OIdJobPost")]
    oid_job_post: String,
    // 职级	OIdJobLevel	string	✔️	✔️	Lookup
    #[serde(default, rename = "OIdJobLevel")]
    oid_job_level: String,
    #[serde(default, rename = "Place")]
    // 工作地点	Place	string	✔️	✔️
    place: String,
    // 入职日期	EntryDate	dateTime	✔️	✔️
    #[serde(rename = "EntryDate", with = "date_format")]
    entry_date: DateTime<Utc>,
    // 生效日期	StartDate	dateTime	✔️	✔️
    #[serde(rename = "StartDate", with = "date_format")]
    start_date: DateTime<Utc>,
    // 失效日期	StopDate	dateTime	✔️	✔️
    #[serde(rename = "StopDate", with = "date_format")]
    stop_date: DateTime<Utc>,
    // 人员状态	EmployeeStatus	string	❌	✔️
    #[serde(default, rename = "EmployeeStatus")]
    employee_status: String,
    // 工号	JobNumber	string	✔️	✔️
    #[serde(default, rename = "JobNumber")]
    work_number: String,
    // 虚线经理	POIdEmpReserve2	integer	✔️	✔️
    #[serde(default, rename = "POIdEmpAdmin")]
    poid_emp_reserve2: u32,
    // 工龄校正值	WorkYearBefore	float	✔️	✔️
    #[serde(default, rename = "WorkYearBefore")]
    work_year_before: f32,
    // 司龄校正值	WorkYearCompanyBefore	float	✔️	✔️
    // 该字段用于调整最终累计司龄，如果设置了离职重聘累计司龄或者实习期间累计司龄，
    // 则在离职重聘和实习生转正时系统会自动计算出需累计期间的司龄并赋值给司龄校正值；
    // 在职人员的最终累计司龄=当前日期-入职日期+司龄校正值。
    #[serde(default, rename = "WorkYearCompanyBefore")]
    work_year_company_before: f32,
    // 累计工龄	WorkYearTotal	float	✔️	✔️
    #[serde(default, rename = "WorkYearTotal")]
    work_year_total: f32,
    // 累计司龄	WorkYearCompanyTotal	float	✔️	✔️
    #[serde(default, rename = "WorkYearCompanyTotal")]
    work_year_company_total: f32,
    #[serde(
        default = "datetime::default",
        rename = "ProbationStartDate",
        with = "date_format"
    )]
    // 试用开始日期	ProbationStartDate	dateTime	✔️	✔️
    probation_start_date: DateTime<Utc>,
    #[serde(
        default = "datetime::default",
        rename = "ProbationStopDate",
        with = "date_format"
    )]
    // 预计试用结束日期	ProbationStopDate	dateTime	✔️	✔️
    probation_stop_date: DateTime<Utc>,
    // 实际试用结束日期	ProbationActualStopDate	dateTime	✔️	✔️
    #[serde(
        default = "datetime::default",
        rename = "ProbationActualStopDate",
        with = "date_format"
    )]
    probation_actual_stop_date: DateTime<Utc>,
    // 试用期(月)	Probation	integer	✔️	✔️
    #[serde(default, rename = "Probation")]
    probation: u8,
    // 转正日期	RegularizationDate	dateTime	✔️	✔️
    #[serde(
        default = "datetime::default",
        rename = "RegularizationDate",
        with = "date_format"
    )]
    regularization_date: DateTime<Utc>,
    // 顺序号	Order	integer	✔️	✔️
    #[serde(default, rename = "Order")]
    order: u32,
    // 人员来源	EmploymentSource	string	✔️	✔️
    #[serde(default, rename = "EmploymentSource")]
    employment_source: String,
    // 用工形式	EmploymentForm	string	✔️	✔️
    #[serde(default, rename = "EmploymentForm")]
    employment_form: String,
    // 是否部门负责人	IsCharge	string	✔️	✔️
    #[serde(default, rename = "IsCharge")]
    is_charge: String,
    // 业务类型	BusinessTypeOID	string	❌	✔️
    #[serde(default, rename = "BusinessTypeOID")]
    business_type_oid: String,
    // 变动类型	ChangeTypeOID	string	✔️	✔️
    #[serde(default, rename = "ChangeTypeOID")]
    change_type_oid: String,
    // 变动原因	ChangeReason	string	✔️	✔️
    #[serde(default, rename = "ChangeReason")]
    change_reason: String,
    // 变动说明	ChangeDesc	string	✔️	✔️	变动说明
    #[serde(default, rename = "ChangeDesc")]
    change_desc: String,
    // 变动后状态	ChangedStatus	string	❌	✔️
    #[serde(default, rename = "ChangedStatus")]
    changed_status: String,
    // 审批状态	ApprovalStatus	string	❌	✔️
    #[serde(default, rename = "ApprovalStatus")]
    approval_status: String,
    // 雇佣关系	EmployType	string	✔️	✔️
    #[serde(default, rename = "EmployType")]
    employ_type: String,
    // 任职类型	ServiceType	string	❌	✔️
    #[serde(default, rename = "ServiceType")]
    service_type: String,
    // 任职状态	ServiceStatus	string	❌	✔️
    #[serde(default, rename = "ServiceStatus")]
    service_status: String,
    // 职务序列	OIdJobSequence	integer	✔️	✔️
    #[serde(default, rename = "OIdJobSequence")]
    oid_job_sequence: String,
    // 是否当前生效	IsCurrentRecord	boolean	❌	✔️
    #[serde(default, rename = "IsCurrentRecord")]
    is_current_record: bool,
    // 最后工作日	LastWorkDate	dateTime	✔️	✔️
    #[serde(
        default = "datetime::default",
        rename = "LastWorkDate",
        with = "date_format"
    )]
    last_work_date: DateTime<Utc>,
    // OId	OId	string	❌	✔️
    #[serde(rename = "OId")]
    oid: String,
    // 创建人	CreatedBy	integer	❌	✔️
    #[serde(rename = "CreatedBy")]
    created_by: u32,
    // 修改人	ModifiedBy	integer	❌	✔️
    #[serde(rename = "ModifiedBy")]
    modified_by: u32,
    // 创建时间	CreatedTime	dateTime	❌	✔️
    #[serde(rename = "CreatedTime", with = "date_format")]
    created_time: DateTime<Utc>,
    // 修改时间	ModifiedTime	dateTime	❌	✔️
    #[serde(rename = "ModifiedTime", with = "date_format")]
    modified_time: DateTime<Utc>,
    // 是否删除	IsDeleted	boolean	❌	✔️
    #[serde(rename = "IsDeleted")]
    is_deleted: bool,
    // 入职日期(年)	EntryDateYear	long	✔️	✔️
    #[serde(default, rename = "EntryDateYear")]
    entry_date_year: u32,
    // 入职状态	EntryStatus	string	✔️	✔️
    #[serde(default, rename = "EntryStatus")]
    entry_status: String,
    // 入职日期(月)	EntryDateMonth	long	✔️	✔️
    #[serde(default, rename = "EntryDateMonth")]
    entry_date_month: u32,
    // 入职日期(日)	EntryDateDay	long	✔️	✔️
    #[serde(default, rename = "EntryDateDay")]
    entry_date_day: u32,
    // 入职准备	EntryPreparationStatus	string	✔️	✔️	入职准备状态
    #[serde(default, rename = "EntryPreparationStatus")]
    entry_preparation_status: String,
    // 调动状态	TransferStatus	string	❌	✔️
    #[serde(default, rename = "TransferStatus")]
    transfer_status: String,
    // 是否有试用期	IsHaveProbation	string	❌	✔️	是否有试用期
    #[serde(default, rename = "IsHaveProbation")]
    is_have_probation: String,
    // 同步到履历	TransferSyncToJobHistory	boolean	✔️	✔️	同步到履历（针对调动）
    #[serde(default, rename = "TransferSyncToJobHistory")]
    transfer_sync_to_job_history: bool,
    // 加入黑名单	AddOrNotBlackList	boolean	✔️	✔️
    #[serde(default, rename = "AddOrNotBlackList")]
    add_or_not_black_list: bool,
    // 加黑原因	BlackAddReason	string	✔️	✔️
    #[serde(default, rename = "BlackAddReason")]
    black_add_reason: String,
    // 加黑说明	BlackStaffDesc	string	✔️	✔️
    #[serde(default, rename = "BlackStaffDesc")]
    black_staff_desc: String,
    // 加黑原因	BlackListAddReason	string	✔️	✔️
    #[serde(default, rename = "BlackListAddReason")]
    black_list_add_reason: String,
    // 是否是插入的任职记录	IsInserted	boolean	✔️	✔️	如果是插入的任职记录则为true,否则是false
    #[serde(default, rename = "IsInserted")]
    is_inserted: bool,
    // 工号（不分词）	JobNumberV2	string	❌	✔️
    #[serde(default, rename = "JobNumberV2")]
    job_number_v2: String,
    // 被复制的任职记录ID(系统用)	FromID	string	✔️	✔️
    #[serde(default, rename = "FromID")]
    from_id: String,
    // LU_入职办理记录	LuEntryAffairsRecord	string	✔️	✔️
    #[serde(default, rename = "LuEntryAffairsRecord")]
    lu_entry_affairs_record: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmployeeBasicInfo {
    // 附加字段
    #[serde(default, rename = "extssyh_108579_1632327139")]
    bank_name: String,
    #[serde(default, rename = "extyhkh_108579_1568158414")]
    bank_account: String,
    #[serde(rename = "Name")]
    // 姓名	Name	string	✔️	✔️
    name: String,
    #[serde(rename = "BusinessModifiedTime", with = "date_format")]
    // 业务修改时间	BusinessModifiedTime	dateTime	✔️	✔️	最后的业务修改时间
    business_modified_time: DateTime<Utc>,
    #[serde(rename = "BusinessModifiedBy")]
    // 业务修改人	BusinessModifiedBy	integer	✔️	✔️	最后的业务修改人
    business_modified_by: u32,
    // 电子邮件	Email	string	✔️	✔️
    #[serde(default, rename = "Email")]
    email: String,
    // 个人邮箱	BackupMail	string	✔️	✔️
    #[serde(default, rename = "BackupMail")]
    personal_mail: String,
    #[serde(default, rename = "IDNumber")]
    // 证件号码	IDNumber	string	✔️	✔️
    id_number: String,
    // 年龄	Age	double	✔️	✔️
    #[serde(default, rename = "Age")]
    age: f32,
    // 性别	Gender	integer	✔️	✔️
    #[serde(default, rename = "Gender")]
    gender: u8,
    // 民族	Nation	string	✔️	✔️
    #[serde(default, rename = "Nation")]
    nation: String,
    // 婚姻状况	MarryCategory	string	✔️	✔️
    #[serde(default, rename = "MarryCategory")]
    marry_category: String,
    // 毕业学校名称	LastSchool	string	✔️	✔️
    #[serde(default, rename = "LastSchool")]
    last_school: String,
    // 专业	Major	string	✔️	✔️
    #[serde(default, rename = "Major")]
    major: String,
    // 国籍(地区)	Nationality	string	✔️	✔️
    #[serde(default, rename = "Nationality")]
    nationality: String,
    // 办公电话	OfficeTel	string	✔️	✔️
    #[serde(default, rename = "OfficeTel")]
    office_tel: Option<String>,
    // 户籍所在地	Birthplace	string	✔️	✔️
    #[serde(default, rename = "Birthplace")]
    birthplace: String,
    // 籍贯	RegistAddress	string	✔️	✔️
    #[serde(default, rename = "RegistAddress")]
    regist_address: String,
    // 联系地址	HomeAddress	string	✔️	✔️
    #[serde(default, rename = "HomeAddress")]
    home_address: String,
    // 最高学历	EducationLevel	string	✔️	✔️
    #[serde(default, rename = "EducationLevel")]
    education_level: String,
    // 手机号码	MobilePhone	string	✔️	✔️
    #[serde(default, rename = "MobilePhone")]
    mobile_phone: String,
    // 联系电话	EmergencyContactPhone	string	✔️	✔️
    #[serde(default, rename = "EmergencyContactPhone")]
    emergency_contact_phone: String,
    // 紧急联系人	EmergencyContact	string	✔️	✔️
    #[serde(default, rename = "EmergencyContact")]
    emergency_contact: String,
    // 与本人关系	EmergencyContactRelationship	string	✔️	✔️
    #[serde(default, rename = "EmergencyContactRelationship")]
    emergency_contact_relationship: String,
    // 户口类别	DomicileType	integer	✔️	✔️
    #[serde(default, rename = "DomicileType")]
    domicile_type: u8,
    // 是否删除(废弃)	IsDeleted	boolean	❌	✔️
    #[serde(rename = "IsDeleted")]
    is_deleted: bool,
    // 证件类型	IDType	string	✔️	✔️
    #[serde(default, rename = "IDType")]
    id_type: String,
    // BeisenUserID	UserID	integer	❌	✔️	北森用户ID
    #[serde(rename = "UserID")]
    user_id: u32,
    // 创建人	CreatedBy	integer	❌	✔️	此字段为系统自动创建，请勿删除
    #[serde(rename = "CreatedBy")]
    created_by: u32,
    // 创建时间	CreatedTime	dateTime	❌	✔️
    #[serde(rename = "CreatedTime", with = "date_format")]
    created_time: DateTime<Utc>,
    // 修改人	ModifiedBy	integer	❌	✔️
    #[serde(rename = "ModifiedBy")]
    modified_by: u32,
    // 修改时间	ModifiedTime	dateTime	❌	✔️
    #[serde(rename = "ModifiedTime", with = "date_format")]
    modified_time: DateTime<Utc>,
    #[serde(default, rename = "StdIsDeleted")]
    // 删除状态	StdIsDeleted	boolean	❌	✔️	此字段为系统自动创建，请勿删除
    std_is_deleted: bool,
    // 参加工作日期(年)	WorkDateYear	long	✔️	✔️
    #[serde(default, rename = "WorkDateYear")]
    // 参加工作日期(月)	WorkDateMonth	long	✔️	✔️
    work_date_year: u32,
    #[serde(default, rename = "WorkDateMonth")]
    work_date_month: u32,
    #[serde(default, rename = "WorkDateDay")]
    // 参加工作日期(日)	WorkDateDay	long	✔️	✔️
    work_date_day: u32,
    // 显示姓名	DisplayName	string	✔️	✔️	姓名（其他语言姓名）
    #[serde(default, rename = "DisplayName")]
    display_name: String,
    // 毕业学校	LastSchoolCode	string	✔️	✔️
    #[serde(default, rename = "LastSchoolCode")]
    last_school_code: Option<String>,
    // 操作类型	EmpInfoOperationType	string	✔️	✔️	标识是否来源为信息采集提交的员工信息
    #[serde(default, rename = "EmpInfoOperationType")]
    emp_info_operation_type: String,
    // 进入公司日期	EntryDate	dateTime	✔️	✔️	记录员工进入该公司的日期
    #[serde(
        default = "datetime::default",
        rename = "EntryDate",
        with = "date_format"
    )]
    entry_date: DateTime<Utc>,
    // 首次进入公司日期	FirstEntryDate	dateTime	✔️	✔️	记录员工首次进入该公司的日期
    #[serde(
        default = "datetime::default",
        rename = "FirstEntryDate",
        with = "date_format"
    )]
    first_entry_date: DateTime<Utc>,
    // 最新进入公司日期	LatestEntryDate	dateTime	✔️	✔️	记录员工最后一次进入本公司的日期
    #[serde(
        default = "datetime::default",
        rename = "LatestEntryDate",
        with = "date_format"
    )]
    latest_entry_date: DateTime<Utc>,
    // 工号	JobNumber	string	✔️	✔️
    #[serde(default, rename = "JobNumber")]
    work_number: String,
}
