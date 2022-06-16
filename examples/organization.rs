use dotenv::dotenv;
use std::env;
use tracing::info;

use beisen::Organizationer;

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
    //     .search_organization_by_ids(vec![843382, 843395, 843409])
    //     .await?;
    // info!("{:?}", r);
    let kind = beisen::dto::TimeWindowOptionKind::Organization;
    let opt = beisen::dto::SearchTimeWindowOption::new_with_days(kind, 1, 360);
    let r = client.search_organization_with_timewindow(&opt).await?;
    info!("{:?}", r.len());
    Ok(())
}
