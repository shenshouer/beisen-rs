use dotenv::dotenv;
use std::env;
use tracing::info;

use beisen::JobPositioner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = beisen::Client::new(
        env::var("APP_ID")?.parse()?,
        env::var("SECRET")?,
        env::var("TENANT_ID")?.parse()?,
    );

    // let r = client.list_setting(on).await?;
    let r = client
        .search_position_by_ids(vec![
            "161601",
            "051b1ddd-7d49-49e5-bbc5-468dcc5e2168",
            "9833678d-04fb-41c3-8e15-af1da4ab5951",
        ])
        .await?;
    info!("{}", serde_json::to_string(&r).unwrap());
    Ok(())
}
