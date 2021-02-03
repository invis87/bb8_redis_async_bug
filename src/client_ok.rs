#![allow(unused_imports)]
use dotenv::dotenv;

use bb8_redis::{bb8, redis, redis::AsyncCommands, RedisConnectionManager};
use bb8_redis_async_bug::api::some_service_client::SomeServiceClient;
use bb8_redis_async_bug::api::{GetRequest, GetResponse, SetRequest, SetResponse};
use bb8_redis_async_bug::client::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "client_ok=debug,bb8_redis_stuck=debug");
    env_logger::init();

    log::debug!("start service client");

    log::info!("creating service client");
    let mut some_service_client = SomeServiceClient::connect("http://localhost:50061").await?;
    log::info!("service client created");

    set_status_1000_times(&mut some_service_client, 2000, 2200).await;

    Ok(())
}
