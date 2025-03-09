use axum::Json;
use serde::Serialize;
use crate::api::sys_status::StatusCode::Healthy;

pub async fn sys_status() -> Json<SystemStatus> {
    // TODO: Replace placeholder status data with real metrics.
    let status_response = SystemStatus {
        status: Healthy,
        ttv_socket_clients: 0,
        live_streams: 0,
    };

    Json(status_response)
}

#[derive(Serialize)]
pub enum StatusCode {
    Healthy,
}

#[derive(Serialize)]
pub struct SystemStatus {
    status: StatusCode,
    ttv_socket_clients: u8, // 0 - 255
    live_streams: u64,
}
