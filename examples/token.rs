use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::info;

use beisen::Tokener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = beisen::Client::new(
        env::var("APP_ID")?.parse()?,
        env::var("SECRET")?,
        env::var("TENANT_ID")?.parse()?,
    );
    let token = client.access_token().await?;
    info!("{:?}", &token);
    info!("has expired ?: {}", token.is_expires());
    Ok(())
}
