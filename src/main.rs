use std::process::exit;
use crate::http_client::{request_s2s_token, AccessToken};

mod http_server;
mod socket_client;
mod api;
mod http_client;

#[tokio::main]
async fn main() {
    let access_token: AccessToken;

    match request_s2s_token().await {
        Ok(result) => {
            access_token = result
        },
        Err(e) => {
            println!("Error while fetching access token: {}", e);
            exit(1)
        }
    }

    println!("Access token: {:?}", access_token);

    let api_server =  async  {
        http_server::serve_api().await;
    };

    let event_service = async {
        // stub
    };

    tokio::join!(event_service, api_server);
}
