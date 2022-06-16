use dotenv::dotenv;
use std::env;
use tracing::info;

use beisen::{dto::ObjectName, Settinger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = beisen::Client::new(
        env::var("APP_ID")?.parse()?,
        env::var("SECRET")?,
        env::var("TENANT_ID")?.parse()?,
    );

    let on = ObjectName::JobLevel;
    // let r = client.list_setting(on).await?;
    let r = client
        .search_setting_by_ids(
            on,
            vec![
                "a8c1fcf5-c068-42d4-80e9-fe202b055a0c",
                "051b1ddd-7d49-49e5-bbc5-468dcc5e2168",
                "9833678d-04fb-41c3-8e15-af1da4ab5951",
            ],
        )
        .await?;
    info!("{}", serde_json::to_string(&r).unwrap());
    Ok(())
}
