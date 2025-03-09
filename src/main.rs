mod http_server;
mod socket_client;
mod api;

#[tokio::main]
async fn main() {
    let api_server =  async  {
        http_server::serve_api().await;
    };

    let event_service = async {
        // stub
    };

    tokio::join!(event_service, api_server);
}
