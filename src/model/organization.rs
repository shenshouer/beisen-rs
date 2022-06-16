use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::datetime::{self, date_format};

#[derive(Debug, Deserialize, Serialize)]
pub struct Organization {
    #[serde(default, rename = "BroadType")]
    broad_type: String,
    #[serde(rename = "BusinessModifiedBy")]
    business_modified_by: u32,
    #[serde(
        default = "datetime::default",
        rename = "BusinessModifiedTime",
        with = "date_format"
    )]
    business_modified_time: DateTime<Utc>,
    #[serde(
        default = "datetime::default",
        rename = "ChangeDate",
        with = "date_format"
    )]
    change_date: DateTime<Utc>,
    #[serde(default, rename = "Code")]
    code: String,
    #[serde(rename = "CreatedBy")]
    created_by: u32,
    #[serde(
        default = "datetime::default",
        rename = "CreatedTime",
        with = "date_format"
    )]
    created_time: DateTime<Utc>,
    #[serde(
        default = "datetime::default",
        rename = "EstablishDate",
        with = "date_format"
    )]
    establish_date: DateTime<Utc>,
    #[serde(default, rename = "FirstLevelOrganization")]
    first_level_organization: u32,
    #[serde(default, rename = "HRBP")]
    hrbp: u32,
    #[serde(default, rename = "IsDeleted")]
    is_deleted: bool,
    #[serde(default, rename = "IsVirtualOrg")]
    is_virtual_org: bool,
    #[serde(default, rename = "Level")]
    level: String,
    #[serde(rename = "ModifiedBy")]
    modified_by: u32,
    #[serde(
        default = "datetime::default",
        rename = "ModifiedTime",
        with = "date_format"
    )]
    modified_time: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "OId")]
    oid: u32,
    #[serde(default, rename = "OIdOrganizationType")]
    oid_organization_type: String,
    #[serde(default, rename = "OrderAdmin")]
    order_admin: u32,
    #[serde(rename = "POIdOrgAdmin")]
    poid_org_admin: i32,
    #[serde(default, rename = "POIdOrgAdminNameTreePath")]
    poid_org_admin_name_tree_path: String,
    #[serde(default, rename = "POIdOrgAdmin_TreeLevel")]
    poid_org_admin_tree_level: i32,
    #[serde(default, rename = "POIdOrgAdmin_TreePath")]
    poid_org_admin_tree_path: String,
    #[serde(default, rename = "POIdOrgReserve2")]
    poid_org_reserve2: i32,
    #[serde(default, rename = "POIdOrgReserve3")]
    poid_org_reserve3: i32,
    #[serde(default, rename = "POIdOrgReserve4")]
    poid_org_reserve4: i32,
    #[serde(default, rename = "POIdOrgReserve5")]
    poid_org_reserve5: i32,
    #[serde(default, rename = "PersonInCharge")]
    person_in_charge: u32,
    #[serde(default, rename = "Place")]
    place: String,
    #[serde(default, rename = "SecondLevelOrganization")]
    second_level_organization: u32,
    #[serde(default, rename = "ShortName")]
    short_name: String,
    #[serde(
        default = "datetime::default",
        rename = "StartDate",
        with = "date_format"
    )]
    start_date: DateTime<Utc>,
    #[serde(default, rename = "Status")]
    status: u8,
    #[serde(default, rename = "StdIsDeleted")]
    std_is_deleted: bool,
    #[serde(
        default = "datetime::default",
        rename = "StopDate",
        with = "date_format"
    )]
    stop_date: DateTime<Utc>,
}
