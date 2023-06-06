use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection, fs, Reply};
use std::path::Path;
use serde::{Deserialize, Serialize};
use postgres::{Client, NoTls};

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

// fn connect() -> Client {
//   return Client::connect("postgres://wyethjackson@localhost:5432/wyethjackson", NoTls).unwrap();
// }

// pub fn migrate() {
//   let mut client = self::connect();
//   // let statement = conn.prepare("SELECT id FROM sqlVersions LIMIT 1 DESC id");
//   // let rows = client.query(&statement);
//   // client.execute("INSERT INTO person (name, data) VALUES ($1, $2)")?;
//
//   for row in client.query("SELECT id FROM sqlVersions LIMIT 1 DESC id", &[])? {
//     let id: i32 = row.get(0);
//
//     println!("found version: {}", id);
//   }
//     // let name: = rows[0];
//     // println!("name: {}", name);
//   // let sql = fs::read_to_string(sql_file).unwrap();
//   //
//   // let results = conn.batch_execute(&sql).unwrap();
//   // println
// }




