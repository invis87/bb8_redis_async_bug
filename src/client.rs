#![allow(unused_imports)]
use dotenv::dotenv;

use crate::api::some_service_client::SomeServiceClient;
use crate::api::{GetRequest, GetResponse, SetRequest, SetResponse};
use bb8_redis::{bb8, redis, redis::AsyncCommands, RedisConnectionManager};

pub async fn get_status_1000_times_with_new_client(
    from: u64,
    to: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    for i in from..to {
        let mut service_client = SomeServiceClient::connect("http://localhost:50061").await?;
        log::debug!("getting status, id: {}", i);
        let get_request = tonic::Request::new(GetRequest { id: i });
        let response = service_client.get(get_request).await?;
        log::debug!("get status response: {:?}", response);
    }

    Ok(())
}

pub async fn set_status_1000_times_with_new_client(
    from: u64,
    to: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    for i in from..to {
        let mut service_client = SomeServiceClient::connect("http://localhost:50061").await?;
        log::debug!("setting status, id: {}", i);
        let set_request = tonic::Request::new(SetRequest { id: i, status: i });
        let response = service_client.set(set_request).await?;
        log::debug!("set status response: {:?}", response);
    }

    Ok(())
}

pub async fn set_status_1000_times(
    service_client: &mut SomeServiceClient<tonic::transport::Channel>,
    from: u64,
    to: u64,
) -> Result<(), tonic::Status> {
    for i in from..to {
        log::debug!("setting status, camera_id: {}", i);
        let set_request = tonic::Request::new(SetRequest { id: i, status: i });
        let response = service_client.set(set_request).await?;
        log::debug!("set status response: {:?}", response);
    }

    Ok(())
}
