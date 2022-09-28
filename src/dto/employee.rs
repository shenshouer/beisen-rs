pub(crate) const REQUEST_EMPLOYEE_COLUMNS: &[&str] = &[
    // 自定义字段
    "extssyh_108579_1632327139", // 收款银行
    "extyhkh_108579_1568158414", // 收款账户
    // 员工信息
    "Name",                          // 姓名	Name	string	✔️	✔️
    "BusinessModifiedTime",          // 业务修改时间	BusinessModifiedTime	dateTime	✔️	✔️	最后的业务修改时间
    "BusinessModifiedBy",            // 业务修改人	BusinessModifiedBy	integer	✔️	✔️	最后的业务修改人
    "EngName",                       // 其他语言姓名	EngName	string	✔️	✔️
    "Email",                         // 电子邮件	Email	string	✔️	✔️
    "BackupMail",                    // 个人邮箱	BackupMail	string	✔️	✔️
    "IDNumber",                      // 证件号码	IDNumber	string	✔️	✔️
    "PassportNumber",                // 护照号码	PassportNumber	string	✔️	✔️
    "Birthday",                      // 出生日期	Birthday	dateTime	✔️	✔️
    "Age",                           // 年龄	Age	double	✔️	✔️
    "Constellation",                 // 星座	Constellation	integer	✔️	✔️
    "BloodType",                     // 血型	BloodType	integer	✔️	✔️
    "Gender",                        // 性别	Gender	integer	✔️	✔️
    "Nation",                        // 民族	Nation	string	✔️	✔️
    "PoliticalStatus",               // 政治面貌	PoliticalStatus	string	✔️	✔️
    "IDPhoto",                       // 照片	IDPhoto	string	✔️	✔️
    "MarryCategory",                 // 婚姻状况	MarryCategory	string	✔️	✔️
    "LastSchool",                    // 毕业学校名称	LastSchool	string	✔️	✔️
    "Major",                         // 专业	Major	string	✔️	✔️
    "Nationality",                   // 国籍(地区)	Nationality	string	✔️	✔️
    "OfficeTel",                     // 办公电话	OfficeTel	string	✔️	✔️
    "Birthplace",                    // 户籍所在地	Birthplace	string	✔️	✔️
    "RegistAddress",                 // 籍贯	RegistAddress	string	✔️	✔️
    "PostalCode",                    // 邮政编码	PostalCode	string	✔️	✔️
    "HomeAddress",                   // 联系地址	HomeAddress	string	✔️	✔️
    "EducationLevel",                // 最高学历	EducationLevel	string	✔️	✔️
    "GraduateDate",                  // 毕业时间	GraduateDate	dateTime	✔️	✔️
    "HomePhone",                     // 家庭电话	HomePhone	string	✔️	✔️
    "BusinessAddress",               // 办公地址	BusinessAddress	string	✔️	✔️
    "MobilePhone",                   // 手机号码	MobilePhone	string	✔️	✔️
    "EmergencyContactPhone",         // 联系电话	EmergencyContactPhone	string	✔️	✔️
    "EmergencyContact",              // 紧急联系人	EmergencyContact	string	✔️	✔️
    "EmergencyContactRelationship",  // 与本人关系	EmergencyContactRelationship	string	✔️	✔️
    "PictureUrl",                    // 头像	PictureUrl	string	✔️	✔️
    "WeiXin",                        // 微信	WeiXin	string	✔️	✔️
    "QQ",                            // QQ	QQ	string	✔️	✔️
    "MSN",                           // MSN	MSN	string	✔️	✔️
    "PersonalHomepage",              // 个人主页	PersonalHomepage	string	✔️	✔️
    "Speciality",                    // 特长	Speciality	string	✔️	✔️
    "Skype",                         // Skype	Skype	string	✔️	✔️
    "GTalk",                         // GTalk	GTalk	string	✔️	✔️
    "DomicileType",                  // 户口类别	DomicileType	integer	✔️	✔️
    "Blog",                          // Blog链接地址	Blog	string	✔️	✔️
    "WorkEmail",                     // 电子邮件（工作）	WorkEmail	string	✔️	✔️
    "ResidenceAddress",              // 户籍地址	ResidenceAddress	string	✔️	✔️
    "JoinPartyDate",                 // 入党/团日期	JoinPartyDate	dateTime	✔️	✔️
    "WorkDate",                      // 参加工作日期	WorkDate	dateTime	✔️	✔️
    "Region",                        // 区域	Region	integer	✔️	✔️
    "AboutMe",                       // 关于我	AboutMe	string	✔️	✔️
    "UserMark",                      // 用户标识	UserMark	string	✔️	✔️
    "BindMsn",                       // MSN绑定的机器人帐号	BindMsn	string	✔️	✔️
    "IsDeleted",                     // 是否删除(废弃)	IsDeleted	boolean	❌	✔️
    "ApprovalStatus",                // 审批状态	ApprovalStatus	string	❌	✔️
    "IDType",                        // 证件类型	IDType	string	✔️	✔️
    "UserID",                        // BeisenUserID	UserID	integer	❌	✔️	北森用户ID
    "CreatedBy",                     // 创建人	CreatedBy	integer	❌	✔️	此字段为系统自动创建，请勿删除
    "CreatedTime",                   // 创建时间	CreatedTime	dateTime	❌	✔️
    "ModifiedBy",                    // 修改人	ModifiedBy	integer	❌	✔️
    "ModifiedTime",                  // 修改时间	ModifiedTime	dateTime	❌	✔️
    "StdIsDeleted",                  // 删除状态	StdIsDeleted	boolean	❌	✔️	此字段为系统自动创建，请勿删除
    "Synopsis",                      // 简介	Synopsis	string	✔️	✔️	简介
    "WorkDateYear",                  // 参加工作日期(年)	WorkDateYear	long	✔️	✔️
    "WorkDateMonth",                 // 参加工作日期(月)	WorkDateMonth	long	✔️	✔️
    "WorkDateDay",                   // 参加工作日期(日)	WorkDateDay	long	✔️	✔️
    "LUSalaryProfile",               // LU_薪资档案(废弃)	LUSalaryProfile	string	❌	✔️
    "OrderCode",                     // 排序编码	OrderCode	long	❌	✔️
    "InviteForActivation",           // 邀请激活账号	InviteForActivation	boolean	✔️	✔️
    "ResumeInformation",             // 简历信息	ResumeInformation	string	✔️	✔️
    "IDPortraitSide",                // 身份证人像面	IDPortraitSide	string	✔️	✔️
    "CertificateValidityTerm",       // 证件有效期	CertificateValidityTerm	dateTime	✔️	✔️
    "IDCountryEmblemSide",           // 身份证国徽面	IDCountryEmblemSide	string	✔️	✔️
    "FieldExtension",                // 前端扩展字段	FieldExtension	string	✔️	✔️
    "UserIdsFromAssesment", // 测评人员id	UserIdsFromAssesment	string	✔️	✔️	测评人员的UserId，由于他们的人员id与我们的UserId是多对一的关系，然后多人选择控件字段也无法做关联关系，所以先跟人才管理的魏玉超同学沟通结论是创建为文本。
    "UserIdsFromI360", // 360员工id	UserIdsFromI360	string	✔️	✔️	360人员的UserId，由于他们的人员id与我们的UserId是多对一的关系，然后多人选择控件字段也无法做关联关系，所以先跟人才管理的魏玉超同学沟通结论是创建为文本。
    "EmailsFromI360", // 360员工邮箱	EmailsFromI360	string	✔️	✔️	360员工的邮箱，可能是多个，所以没有把字段类型设置为电子邮件。
    "ReferrerUserID", // 推荐人	ReferrerUserID	integer	✔️	✔️
    "DisplayName",    // 显示姓名	DisplayName	string	✔️	✔️	姓名（其他语言姓名）
    "PlaceOfBirth",   // 出生地	PlaceOfBirth	string	✔️	✔️	员工信：出生地
    "HealthStatusSort", // 健康状况	HealthStatusSort	string	✔️	✔️	HealthStatusSort
    "TechnicalLevelSort", // 专业技术级别	TechnicalLevelSort	string	✔️	✔️	专业技术级别
    "SpecialitySkill", // 熟悉专业有何专长	SpecialitySkill	string	✔️	✔️	熟悉专业有何专长
    "TechnicalPostName", // 专业技术职务任职资格名称	TechnicalPostName	string	✔️	✔️	专业技术职务任职资格名称
    "IsTeamMembers",     // 是否班子成员	IsTeamMembers	boolean	✔️	✔️	国央企-是否班子成员
    "IsReturnHome",      // 是否留学回国人员	IsReturnHome	boolean	✔️	✔️	国央企-是否留学回国人员
    "HealthStatusID",    // 健康状况	HealthStatusID	string	✔️	✔️
    "TechnicalLevelID",  // 专业技术级别	TechnicalLevelID	string	✔️	✔️	专业技术级别
    "TechnicalPostQualificationsID", // 专业技术职务任职资格名称	TechnicalPostQualificationsID	string	✔️	✔️	国央企-专业技术职务任职资格名称
    "InterviewEvaluation",           // 面试评价	InterviewEvaluation	string	✔️	✔️
    "ManagementTalentCategoryID",    // 管理人才分类	ManagementTalentCategoryID	string	✔️	✔️	管理人才分类
    "EmpInfoCurrentApprovalUserID",  // 当前审批人	EmpInfoCurrentApprovalUserID	string	✔️	✔️
    "ApplicantIdV2",                 // 应聘者(数据类型置换临时用字段)	ApplicantIdV2	integer	✔️	✔️
    "ApplicantIdV3", // 应聘者(数据类型置换临时用字段)	ApplicantIdV3	string	✔️	✔️	用于招聘应聘者数据同步
    "LastSchoolCode", // 毕业学校	LastSchoolCode	string	✔️	✔️
    "EmpInfoOperationType", // 操作类型	EmpInfoOperationType	string	✔️	✔️	标识是否来源为信息采集提交的员工信息
    "InServiceCollectInfoLookup", // 员工信息采集Lookup	InServiceCollectInfoLookup	string	✔️	✔️
    "FirstEntryDate",       // 首次进入公司日期	FirstEntryDate	dateTime	✔️	✔️	记录员工首次进入该公司的日期
    "LatestEntryDate", // 最新进入公司日期	LatestEntryDate	dateTime	✔️	✔️	记录员工最后一次进入本公司的日期
    // 任职记录（EmploymentRecord）
    "UserID",                       // 人员	UserID	integer	❌	✔️
    "BusinessModifiedBy",           // 业务修改人	BusinessModifiedBy	integer	✔️	✔️	最后的业务修改人
    "BusinessModifiedTime",         // 业务修改时间	BusinessModifiedTime	dateTime	✔️	✔️	最后的业务修改时间
    "OIdOrganization",              // 机构	OIdOrganization	integer	✔️	✔️
    "StdIsDeleted",                 // 删除状态	StdIsDeleted	boolean	❌	✔️	此字段为系统自动创建，请勿删除
    "OIdDepartment",                // 部门	OIdDepartment	integer	✔️	✔️
    "OIdJobPosition",               // 职位	OIdJobPosition	string	✔️	✔️
    "OIdJobPost",                   // 职务	OIdJobPost	integer	✔️	✔️
    "OIdJobLevel",                  // 职级	OIdJobLevel	string	✔️	✔️	Lookup
    "OidJobGrade",                  // 职等	OidJobGrade	string	✔️	✔️
    "Place",                        // 工作地点	Place	string	✔️	✔️
    "EntryDate",                    // 入职日期	EntryDate	dateTime	✔️	✔️
    "StartDate",                    // 生效日期	StartDate	dateTime	✔️	✔️
    "StopDate",                     // 失效日期	StopDate	dateTime	✔️	✔️
    "EmployeeStatus",               // 人员状态	EmployeeStatus	string	❌	✔️
    "JobNumber",                    // 工号	JobNumber	string	✔️	✔️
    "POIdEmpAdmin",                 // 直线经理	POIdEmpAdmin	integer	✔️	✔️
    "POIdEmpReserve2",              // 虚线经理	POIdEmpReserve2	integer	✔️	✔️
    "WorkYearBefore",               // 工龄校正值	WorkYearBefore	float	✔️	✔️
    "WorkYearCompanyBefore", // 司龄校正值	WorkYearCompanyBefore	float	✔️	✔️	该字段用于调整最终累计司龄，如果设置了离职重聘累计司龄或者实习期间累计司龄，则在离职重聘和实习生转正时系统会自动计算出需累计期间的司龄并赋值给司龄校正值；在职人员的最终累计司龄=当前日期-入职日期+司龄校正值。
    "WorkYearTotal",         // 累计工龄	WorkYearTotal	float	✔️	✔️
    "WorkYearCompanyTotal",  // 累计司龄	WorkYearCompanyTotal	float	✔️	✔️
    "ProbationStartDate",    // 试用开始日期	ProbationStartDate	dateTime	✔️	✔️
    "ProbationStopDate",     // 预计试用结束日期	ProbationStopDate	dateTime	✔️	✔️
    "ProbationActualStopDate", // 实际试用结束日期	ProbationActualStopDate	dateTime	✔️	✔️
    "Probation",             // 试用期(月)	Probation	integer	✔️	✔️
    "RegularizationDate",    // 转正日期	RegularizationDate	dateTime	✔️	✔️
    "ProbationResult",       // 试用结果	ProbationResult	string	✔️	✔️
    "Order",                 // 顺序号	Order	integer	✔️	✔️
    "EmploymentType",        // 人员类别	EmploymentType	string	✔️	✔️
    "EmploymentSource",      // 人员来源	EmploymentSource	string	✔️	✔️
    "EmploymentForm",        // 用工形式	EmploymentForm	string	✔️	✔️
    "IsCharge",              // 是否部门负责人	IsCharge	string	✔️	✔️
    "IdentityLabel",         // 身份类型	IdentityLabel	string	✔️	✔️
    "BusinessTypeOID",       // 业务类型	BusinessTypeOID	string	❌	✔️
    "ChangeTypeOID",         // 变动类型	ChangeTypeOID	string	✔️	✔️
    "ChangeReason",          // 变动原因	ChangeReason	string	✔️	✔️
    "ChangeDesc",            // 变动说明	ChangeDesc	string	✔️	✔️	变动说明
    "ChangedStatus",         // 变动后状态	ChangedStatus	string	❌	✔️
    "TransitionTypeOID",     // 异动类型	TransitionTypeOID	string	✔️	✔️
    "ApprovalStatus",        // 审批状态	ApprovalStatus	string	❌	✔️
    "EmployType",            // 雇佣关系	EmployType	string	✔️	✔️
    "ServiceType",           // 任职类型	ServiceType	string	❌	✔️
    "ServiceStatus",         // 任职状态	ServiceStatus	string	❌	✔️
    "OIdJobSequence",        // 职务序列	OIdJobSequence	integer	✔️	✔️
    "OIdProfessionalLine",   // 专业条线	OIdProfessionalLine	string	✔️	✔️
    "IsCurrentRecord",       // 是否当前生效	IsCurrentRecord	boolean	❌	✔️
    "Whereabouts",           // 离职后去向	Whereabouts	string	✔️	✔️
    "LastWorkDate",          // 最后工作日	LastWorkDate	dateTime	✔️	✔️
    "OId",                   // OId	OId	string	❌	✔️
    "CreatedBy",             // 创建人	CreatedBy	integer	❌	✔️
    "ModifiedBy",            // 修改人	ModifiedBy	integer	❌	✔️
    "CreatedTime",           // 创建时间	CreatedTime	dateTime	❌	✔️
    "ModifiedTime",          // 修改时间	ModifiedTime	dateTime	❌	✔️
    "IsDeleted",             // 是否删除	IsDeleted	boolean	❌	✔️
    "POIdEmpReserve3",       // 预留3	POIdEmpReserve3	integer	✔️	✔️
    "POIdEmpReserve4",       // 预留4	POIdEmpReserve4	integer	✔️	✔️
    "POIdEmpReserve5",       // 预留5	POIdEmpReserve5	integer	✔️	✔️
    "TraineeStartDate",      // 实习开始日期	TraineeStartDate	dateTime	✔️	✔️	实习开始日期
    "Remarks",               // 备注	Remarks	string	✔️	✔️
    "ExtendInfo",            // ExtendInfo	ExtendInfo	string	✔️	✔️	携带拓展参数，内容由功能拓展决定
    "EntryDateYear",         // 入职日期(年)	EntryDateYear	long	✔️	✔️
    "EntryStatus",           // 入职状态	EntryStatus	string	✔️	✔️
    "EntryDateMonth",        // 入职日期(月)	EntryDateMonth	long	✔️	✔️
    "EntryDateDay",          // 入职日期(日)	EntryDateDay	long	✔️	✔️
    "EntryPreparationStatus", // 入职准备	EntryPreparationStatus	string	✔️	✔️	入职准备状态
    "POIdEmpAdminTransmitJunior", // 直线下级转交给	POIdEmpAdminTransmitJunior	long	✔️	✔️
    "POIdEmpReserveTransmitJunior", // 虚线下级转交给	POIdEmpReserveTransmitJunior	long	✔️	✔️
    "DefaultEntryTimeStatus", // 默认入职日期入职	DefaultEntryTimeStatus	boolean	✔️	✔️
    "OrderCode", // 排序编码	OrderCode	long	❌	✔️	用于添加排序编码,该编码是通过配置排序规则,并使用排序编码计算引擎计算出来的一组数值
    "OrderCode1", // 排序编码1	OrderCode1	long	❌	✔️	用于添加排序编码,该编码是通过配置排序规则,并使用排序编码计算引擎计算出来的一组数值
    "LUSalaryProfile", // LU_薪资档案	LUSalaryProfile	string	❌	✔️
    "LUSalaryAdjustment", // LU_调薪管理	LUSalaryAdjustment	string	❌	✔️
    "LUSocialSecurityProfile", // LU_社保档案	LUSocialSecurityProfile	string	❌	✔️
    "TransferStatus", // 调动状态	TransferStatus	string	❌	✔️
    "LUOffer",    // LU_Offer	LUOffer	string	❌	✔️
    "IsHaveProbation", // 是否有试用期	IsHaveProbation	string	❌	✔️	是否有试用期
    "TransferSyncToJobHistory", // 同步到履历	TransferSyncToJobHistory	boolean	✔️	✔️	同步到履历（针对调动）
    "IsLanchElectronicSignature", // 发起电子文件签署废弃	IsLanchElectronicSignature	boolean	✔️	✔️
    "LUEmpChangeRecord", // LU_任职变动记录	LUEmpChangeRecord	string	✔️	✔️	任职记录关联变动记录
    "LookUpSignRecord", // LU_签署记录	LookUpSignRecord	string	✔️	✔️
    "ElectronicSignature", // 发起电子文件签署	ElectronicSignature	boolean	✔️	✔️	开启前，请确保已经开通电子签产品。
    "AddOrNotBlackList",   // 加入黑名单	AddOrNotBlackList	boolean	✔️	✔️
    "BlackAddReason",      // 加黑原因	BlackAddReason	string	✔️	✔️
    "BlackStaffDesc",      // 加黑说明	BlackStaffDesc	string	✔️	✔️
    "BlackListAddReason",  // 加黑原因	BlackListAddReason	string	✔️	✔️
    "DelayProbationStopDate", // 延期后预计试用结束日期	DelayProbationStopDate	dateTime	✔️	✔️
    "IsInserted", // 是否是插入的任职记录	IsInserted	boolean	✔️	✔️	如果是插入的任职记录则为true,否则是false
    "JobNumberV2", // 工号（不分词）	JobNumberV2	string	❌	✔️
    "JobNumberV3", // 工号（废弃）	JobNumberV3	string	✔️	✔️
    "FromID",     // 被复制的任职记录ID(系统用)	FromID	string	✔️	✔️
    "LuEntryAffairsRecord", // LU_入职办理记录	LuEntryAffairsRecord	string	✔️	✔️
    "LuEntryAffairsRecord", // 离职交接LookUp	DimissionHandoverLookUp	string	✔️	✔️
    "DutyTransferLookUp", // 职责转交LookUp	DutyTransferLookUp	string	✔️	✔️
    "OIdJobPostV2", // 职务(关联关系刷数据用，不要动)	OIdJobPostV2	string	✔️	✔️
    "OIdJobPostV3", // 职务（废弃v3）	OIdJobPostV3	string	✔️	✔️
    "EmpRecCurrentApprovalUserID", // 当前审批人	EmpRecCurrentApprovalUserID	string	✔️	✔️
    "BatchTransferID", // 批量调动ID	BatchTransferID	string	✔️	✔️
    "BatchTransferLookUP", // 批量调动LookUp	BatchTransferLookUP	string	✔️	✔️
    "LUBatchTransfer", // 批量调动LookUp	LUBatchTransfer	string	✔️	✔️
    "OIdJobSequenceV2", // 职务序列（关联关系临时过渡用字段）	OIdJobSequenceV2	integer	✔️	✔️
    "OIdJobSequenceV3", // 职务序列（关联关系临时过渡用字段）	OIdJobSequenceV3	string	✔️	✔️
    "IsChangeContract", // 是否变更合同	IsChangeContract	boolean	✔️	✔️
    "IsKeyPerson", // 是否关键人员	IsKeyPerson	boolean	✔️	✔️	是否关键人员
    "TranferSignRecordLookUp", // 调动协议签署记录LookUp	TranferSignRecordLookUp	string	✔️	✔️
    "IsLatestFlagForOcean", // 专用于Ocean的标志位字段	IsLatestFlagForOcean	boolean	✔️	✔️	专用于Ocean的标志位字段，多租赁不维护值，ocean中通过ETL进行定时计算来赋值，用于赵永亮做Ocean报表统计
];

use crate::utils::datetime::date_format;
use chrono::{DateTime, Duration, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchEmployeeTimeWindowOption {
    #[serde(rename(serialize = "EmpStatus"))]
    emp_status: Vec<u8>,
    #[serde(rename(serialize = "EmployType"))]
    employ_type: Vec<u8>,
    #[serde(rename(serialize = "ServiceType"))]
    service_type: Vec<u8>,
    #[serde(rename(serialize = "IsGetLatestRecord"))]
    is_get_latest_record: bool,
    #[serde(rename(serialize = "StartTime"), with = "date_format")]
    start_time: DateTime<Local>,
    #[serde(rename(serialize = "StopTime"), with = "date_format")]
    stop_time: DateTime<Local>,
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

impl SearchEmployeeTimeWindowOption {
    pub fn new(start_time: DateTime<Local>, stop_time: DateTime<Local>, page_index: usize) -> Self {
        Self {
            emp_status: vec![
                1,  // 待入职
                2,  // 入职试用
                3,  // 正式在职
                8,  // 离职
                4,  // 调出
                5,  // 待调入
                6,  // 退休,
                7,  // 退休返聘
                9,  // 亡故
                10, // 结束返聘
                11, // 待返聘
                12, // 非正式。
            ],
            employ_type: vec![
                0, // 内部员工,
                1, // 外部人员,
                2, // 实习生,
            ],
            service_type: vec![
                0, // 主职,
                1, // 兼职,
                3, // 返聘,
                4, // 借调,
                5, // 外派,
            ],
            is_get_latest_record: true,
            start_time,
            stop_time,
            with_disabled: true,
            with_deleted: true,
            page_index,
            page_size: 300,
            columns: REQUEST_EMPLOYEE_COLUMNS,
        }
    }

    // 距离当前N分钟参数
    pub fn new_with_minutes(page_index: usize, minutes: i64) -> Self {
        let stop_time = Local::now();
        let start_time = stop_time - Duration::minutes(minutes);
        SearchEmployeeTimeWindowOption::new(start_time, stop_time, page_index)
    }

    // 距离当前N天参数
    pub fn new_with_days(page_index: usize, days: i64) -> Self {
        let stop_time = Local::now();
        let start_time = stop_time - Duration::days(days);
        SearchEmployeeTimeWindowOption::new(start_time, stop_time, page_index)
    }

    pub fn clone_with_page_index(&self, page_index: usize) -> Self {
        Self {
            page_index,
            emp_status: self.emp_status.clone(),
            employ_type: self.employ_type.clone(),
            service_type: self.service_type.clone(),
            is_get_latest_record: self.is_get_latest_record,
            start_time: self.start_time,
            stop_time: self.stop_time,
            with_disabled: self.with_disabled,
            with_deleted: self.with_deleted,
            page_size: self.page_size,
            columns: REQUEST_EMPLOYEE_COLUMNS,
        }
    }

    pub fn total(&self) -> usize {
        self.page_index * self.page_size
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }
}

// {
//     "BasicInfoFields": {
//       "JoinPartyDate": "4/20/2022",
//       "ResumeInformation": "string",
//       "PersonalHomepage": "string",
//       "TechnicalPostQualificationsID": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "EmergencyContactRelationship": "string",
//       "RegistAddress": "string",
//       "LastSchoolCode": "string",
//       "PlaceOfBirth": "string",
//       "QQ": "string",
//       "SpecialitySkill": "string",
//       "ResidenceAddress": "string",
//       "TechnicalLevelID": "string",
//       "Birthplace": "string",
//       "EmergencyContact": "string",
//       "BloodType": 0,
//       "AboutMe": "string",
//       "BusinessAddress": "string",
//       "Region": 0,
//       "HomeAddress": "string",
//       "Gender": 0,
//       "Synopsis": "string",
//       "InviteForActivation": true,
//       "ApplicantIdV2": 0,
//       "PassportNumber": "string",
//       "WorkDateMonth": 0,
//       "IsTeamMembers": true,
//       "PostalCode": "string",
//       "EmpInfoCurrentApprovalUserID": "string",
//       "PoliticalStatus": "string",
//       "DisplayName": "string",
//       "MobilePhone": "string",
//       "EngName": "string",
//       "LastSchool": "string",
//       "DomicileType": 0,
//       "GTalk": "string",
//       "CertificateValidityTerm": "4/20/2022",
//       "MarryCategory": "string",
//       "OfficeTel": "string",
//       "FieldExtension": "string",
//       "InServiceCollectInfoLookup": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "Birthday": "4/20/2022",
//       "WorkDateDay": 0,
//       "Age": "string",
//       "Email": "user@example.com",
//       "BindMsn": "string",
//       "Nation": "string",
//       "TechnicalPostName": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "WorkEmail": "string",
//       "IsReturnHome": true,
//       "WorkDateYear": 0,
//       "BackupMail": "string",
//       "IDPhoto": "string",
//       "EmpInfoOperationType": "string",
//       "WorkDate": "4/20/2022",
//       "PictureUrl": "string",
//       "Name": "string",
//       "MSN": "string",
//       "InterviewEvaluation": "string",
//       "EmailsFromI360": "string",
//       "UserIdsFromAssesment": "string",
//       "IDNumber": "string",
//       "Constellation": 0,
//       "TechnicalLevelSort": "string",
//       "IDType": "string",
//       "Blog": "string",
//       "HealthStatusID": "string",
//       "IDCountryEmblemSide": "string",
//       "HealthStatusSort": "string",
//       "GraduateDate": "4/20/2022",
//       "BusinessModifiedTime": "2022-04-20T11:55:03.128Z",
//       "WeiXin": "string",
//       "Major": "string",
//       "IDPortraitSide": "string",
//       "ManagementTalentCategoryID": "string",
//       "UserMark": "string",
//       "Skype": "string",
//       "ReferrerUserID": 0,
//       "ApplicantIdV3": "string",
//       "HomePhone": "string",
//       "EmergencyContactPhone": "string",
//       "BusinessModifiedBy": 0,
//       "UserIdsFromI360": "string",
//       "EducationLevel": "string",
//       "Nationality": "string",
//       "Speciality": "string"
//     },
//     "ServiceInfoFields": {
//       "Whereabouts": "string",
//       "OIdJobPosition": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "Place": "string",
//       "FromID": "string",
//       "BusinessModifiedTime": "2022-04-20T11:55:03.128Z",
//       "POIdEmpReserve4": 0,
//       "ProbationActualStopDate": "4/20/2022",
//       "POIdEmpReserveTransmitJunior": 0,
//       "Remarks": "string",
//       "ProbationResult": "string",
//       "IsCharge": "string",
//       "TraineeStartDate": "4/20/2022",
//       "EmploymentType": "string",
//       "IsLanchElectronicSignature": true,
//       "WorkYearBefore": 0,
//       "Order": 0,
//       "EmployType": "string",
//       "EntryDateMonth": 0,
//       "BlackStaffDesc": "string",
//       "OIdJobSequence": 0,
//       "RegularizationDate": "4/20/2022",
//       "EntryDateYear": 0,
//       "IsKeyPerson": true,
//       "OidJobGrade": "string",
//       "ProbationStopDate": "4/20/2022",
//       "LUBatchTransfer": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "OIdJobPostV3": "string",
//       "BlackListAddReason": "string",
//       "LastWorkDate": "4/20/2022",
//       "ChangeReason": "string",
//       "LUEmpChangeRecord": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "EntryStatus": "string",
//       "WorkYearCompanyTotal": 0,
//       "BlackAddReason": "string",
//       "DimissionHandoverLookUp": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "StartDate": "4/20/2022",
//       "DefaultEntryTimeStatus": true,
//       "POIdEmpReserve2": 0,
//       "TransitionTypeOID": "string",
//       "Probation": 0,
//       "BusinessModifiedBy": 0,
//       "OIdJobSequenceV3": "string",
//       "OIdDepartment": 0,
//       "ChangeTypeOID": "string",
//       "EmpRecCurrentApprovalUserID": "string",
//       "IdentityLabel": "string",
//       "OIdJobPost": 0,
//       "EmploymentForm": "string",
//       "OIdJobPostV2": "string",
//       "WorkYearTotal": 0,
//       "OIdProfessionalLine": "string",
//       "JobNumber": "string",
//       "EntryDate": "4/20/2022",
//       "POIdEmpAdmin": 0,
//       "OIdJobSequenceV2": 0,
//       "POIdEmpAdminTransmitJunior": 0,
//       "AddOrNotBlackList": true,
//       "POIdEmpReserve5": 0,
//       "WorkYearCompanyBefore": 0,
//       "ExtendInfo": "string",
//       "LuEntryAffairsRecord": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "EntryPreparationStatus": "string",
//       "ElectronicSignature": true,
//       "OIdJobLevel": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "EntryDateDay": 0,
//       "DutyTransferLookUp": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "LookUpSignRecord": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "TransferSyncToJobHistory": true,
//       "IsChangeContract": true,
//       "POIdEmpReserve3": 0,
//       "StopDate": "4/20/2022",
//       "OIdOrganization": 0,
//       "TranferSignRecordLookUp": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
//       "ProbationStartDate": "4/20/2022",
//       "IsInserted": true,
//       "ChangeDesc": "string",
//       "EmploymentSource": "string"
//     },
//     "ContractFields": {},
//     "SalaryProfileFields": {},
//     "SocialSecurityFields": {},
//     "ExtParams": {}
// }
