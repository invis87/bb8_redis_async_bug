use anyhow::anyhow;
use bb8_redis::{bb8, redis, redis::AsyncCommands, RedisConnectionManager};
use bb8_redis_async_bug::api::some_service_server::SomeService;
use bb8_redis_async_bug::api::{GetRequest, GetResponse, SetRequest, SetResponse};
use log::error;
use std::collections;
use tonic::{Request, Response, Status};

use bb8_redis_async_bug::{Result, StdResult};

use tonic::transport::Server;

use bb8_redis_async_bug::api::some_service_server::SomeServiceServer;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "pool_server=debug");
    env_logger::init();

    let redis_conn_manager = RedisConnectionManager::new("redis://127.0.0.1:6379")?;
    let redis_pool = bb8::Pool::builder().build(redis_conn_manager).await?;
    let some_service_impl = SomeServiceImpl { redis_pool };

    log::info!("starting server...");

    let grpc_service = SomeServiceServer::new(some_service_impl);
    Server::builder()
        .add_service(grpc_service)
        .serve("0.0.0.0:50061".parse()?)
        .await;

    Ok(())
}

#[derive(Debug)]
struct SomeServiceImpl {
    pub redis_pool: bb8::Pool<RedisConnectionManager>,
}

#[tonic::async_trait]
impl SomeService for SomeServiceImpl {
    async fn get(&self, request: Request<GetRequest>) -> StdResult<Response<GetResponse>, Status> {
        log::debug!("Got a get request: {:?}", request);
        let response = get_handler(&self.redis_pool, request.into_inner()).await?;
        Ok(Response::new(response))
    }

    async fn set(&self, request: Request<SetRequest>) -> StdResult<Response<SetResponse>, Status> {
        log::debug!("Got a set request: {:?}", request);
        let response = set_handler(&self.redis_pool, request.into_inner()).await?;
        Ok(Response::new(response))
    }
}

async fn get_handler(
    redis_pool: &bb8::Pool<RedisConnectionManager>,
    request: GetRequest,
) -> Result<GetResponse> {
    let mut conn = redis_pool.get().await?;
    let key = format!("status_{}", request.id);

    let response: redis::RedisResult<collections::HashMap<String, u64>> = conn.hgetall(key).await;

    match response {
        Ok(info) => Ok(GetResponse {
            id: info.get(ID_ATTRIBUTE).cloned().unwrap_or_default(),
            status: info.get(STATUS_ATTRIBUTE).cloned().unwrap_or_default(),
        }),
        Err(err) => {
            error!("fail to get info from redis, reason: '{:?}'", err);
            Err(anyhow!("redis error").into())
        }
    }
}

const ID_ATTRIBUTE: &str = "id";
const STATUS_ATTRIBUTE: &str = "status";

async fn set_handler(
    redis_pool: &bb8::Pool<RedisConnectionManager>,
    request: SetRequest,
) -> Result<SetResponse> {
    let mut conn = redis_pool.get().await?;
    let key = format!("status_{}", request.id);
    let values = [
        (ID_ATTRIBUTE, request.id),
        (STATUS_ATTRIBUTE, request.status),
    ];

    let response: redis::RedisResult<()> = conn.hset_multiple(key, &values).await;
    match response {
        Ok(_) => Ok(SetResponse {}),
        Err(err) => {
            error!("fail to persist info into redis, reason: '{:?}'", err);
            Err(anyhow!("redis error").into())
        }
    }
}
