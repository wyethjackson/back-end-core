use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection, fs, Reply};
use std::path::Path;
use serde::{Deserialize, Serialize};
use postgres::{Client, NoTls};

mod models {
    pub mod user_model;
}

use models::user_model;

#[derive(Debug, Deserialize, Serialize)]
struct UserData {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let hello = warp::path("hello")
        .map(|| "Hello, world!");

    let index = warp::path::end()
        .and(warp::fs::file("./static/index.html"));

    user_model::get_user(1);
      // Define the POST endpoint filter
    let user = warp::path("user")
        .and(warp::post())
        .and(warp::body::json())
        .map(|user_data: UserData| format!("Received user: {:?}", user_data));

    let routes = hello
        .or(index)
        .or(user);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}




