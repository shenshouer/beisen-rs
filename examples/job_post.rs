use dotenv::dotenv;
use std::env;
use tracing::info;

use beisen::JobPoster;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = beisen::Client::new(
        env::var("APP_ID")?.parse()?,
        env::var("SECRET")?,
        env::var("TENANT_ID")?.parse()?,
    );

    let kind = beisen::dto::TimeWindowOptionKind::JobPost;
    let opt = beisen::dto::SearchTimeWindowOption::new_with_days(kind, 1, 360);
    let r = client.search_job_post_with_timewindow(&opt).await?;
    info!("{:?}", r);
    Ok(())
}
