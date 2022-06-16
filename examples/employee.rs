use dotenv::dotenv;
use std::env;
use tracing::info;

use beisen::Employeer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = beisen::Client::new(
        env::var("APP_ID")?.parse()?,
        env::var("SECRET")?,
        env::var("TENANT_ID")?.parse()?,
    );
    // let r = client
    //     // //.get_uid_by_email("shenshouer2955@ipalfish.com")
    //     //     // .get_basicinfo_by_uids(vec![120736296, 120737316])
    //     //     .get_serviceinfo_by_uids(vec![120736296, 120737316, 153725396])
    //     // .await?;
    // info!("{:?}", r);

    let opt = beisen::dto::SearchEmployeeTimeWindowOption::new_with_minutes(1, 10);
    let r = client.search_employee_with_timewindow(&opt).await?;
    info!("{}", serde_json::to_string(&r)?);
    Ok(())
}
