use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn credits(Query(params): Query<QueryParams>) -> Json<Credits> {
    let _page = if params.page.is_some() { params.page.unwrap() } else { 1 };

    // TODO: some db query here

    // TODO: fill response from db query results
    let credits_response = Credits {
        users: Vec::from([]),
        has_more: false,
    };

    Json(credits_response)
}

#[derive(Deserialize)]
pub enum CreditType {
    Follower,
    Subscriber,
    Cheerer,
    Chatter,
}

#[derive(Deserialize)]
pub struct QueryParams {
    credit_type: CreditType,
    page: Option<u8>,
}

#[derive(Serialize)]
pub struct Credits {
    users: Vec<String>,
    has_more: bool,
}

