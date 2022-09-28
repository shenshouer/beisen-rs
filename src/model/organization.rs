use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct Organization {
    #[serde(default, rename = "BroadType")]
    pub broad_type: String,
    #[serde(rename = "BusinessModifiedBy")]
    pub business_modified_by: u32,
    #[serde(
        default = "datetime::default",
        rename = "BusinessModifiedTime",
        with = "date_format"
    )]
    pub business_modified_time: DateTime<Utc>,
    #[serde(
        default = "datetime::default",
        rename = "ChangeDate",
        with = "date_format"
    )]
    pub change_date: DateTime<Utc>,
    #[serde(default, rename = "Code")]
    pub code: String,
    #[serde(rename = "CreatedBy")]
    pub created_by: u32,
    #[serde(
        default = "datetime::default",
        rename = "CreatedTime",
        with = "date_format"
    )]
    pub created_time: DateTime<Utc>,
    #[serde(
        default = "datetime::default",
        rename = "EstablishDate",
        with = "date_format"
    )]
    pub establish_date: DateTime<Utc>,
    #[serde(default, rename = "FirstLevelOrganization")]
    pub first_level_organization: u32,
    #[serde(default, rename = "HRBP")]
    pub hrbp: u32,
    #[serde(default, rename = "IsDeleted")]
    pub is_deleted: bool,
    #[serde(default, rename = "IsVirtualOrg")]
    pub is_virtual_org: bool,
    #[serde(default, rename = "Level")]
    pub level: String,
    #[serde(rename = "ModifiedBy")]
    pub modified_by: u32,
    #[serde(
        default = "datetime::default",
        rename = "ModifiedTime",
        with = "date_format"
    )]
    pub modified_time: DateTime<Utc>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OId")]
    pub oid: u32,
    #[serde(default, rename = "OIdOrganizationType")]
    pub oid_organization_type: String,
    #[serde(default, rename = "OrderAdmin")]
    pub order_admin: u32,
    #[serde(rename = "POIdOrgAdmin")]
    pub poid_org_admin: i32,
    #[serde(default, rename = "POIdOrgAdminNameTreePath")]
    pub poid_org_admin_name_tree_path: String,
    #[serde(default, rename = "POIdOrgAdmin_TreeLevel")]
    pub poid_org_admin_tree_level: i32,
    #[serde(default, rename = "POIdOrgAdmin_TreePath")]
    pub poid_org_admin_tree_path: String,
    #[serde(default, rename = "POIdOrgReserve2")]
    pub poid_org_reserve2: i32,
    #[serde(default, rename = "POIdOrgReserve3")]
    pub poid_org_reserve3: i32,
    #[serde(default, rename = "POIdOrgReserve4")]
    pub poid_org_reserve4: i32,
    #[serde(default, rename = "POIdOrgReserve5")]
    pub poid_org_reserve5: i32,
    #[serde(default, rename = "PersonInCharge")]
    pub person_in_charge: u32,
    #[serde(default, rename = "Place")]
    pub place: String,
    #[serde(default, rename = "SecondLevelOrganization")]
    pub second_level_organization: u32,
    #[serde(default, rename = "ShortName")]
    pub short_name: String,
    #[serde(
        default = "datetime::default",
        rename = "StartDate",
        with = "date_format"
    )]
    pub start_date: DateTime<Utc>,
    #[serde(default, rename = "Status")]
    pub status: u8,
    #[serde(default, rename = "StdIsDeleted")]
    pub std_is_deleted: bool,
    #[serde(
        default = "datetime::default",
        rename = "StopDate",
        with = "date_format"
    )]
    pub stop_date: DateTime<Utc>,
}
