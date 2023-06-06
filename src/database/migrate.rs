extern crate postgres;

use postgres::{Client, NoTls};
use std::fs;

fn connect() -> Client {
  return Client::connect("postgres://wyethjackson@localhost:5432/wyethjackson", NoTls).unwrap();
}

pub fn migrate(sql_file: String) {
  let mut client = self::connect();
  // let statement = conn.prepare("SELECT id FROM sqlVersions LIMIT 1 DESC id");
  // let rows = client.query(&statement);
  // client.execute("INSERT INTO person (name, data) VALUES ($1, $2)")?;

  for row in client.query("SELECT id FROM sqlVersions LIMIT 1 DESC id", &[])? {
    let id: i32 = row.get(0);

    println!("found version: {}", id);
  }
    // let name: = rows[0];
    // println!("name: {}", name);
  // let sql = fs::read_to_string(sql_file).unwrap();
  //
  // let results = conn.batch_execute(&sql).unwrap();
  // println
}

// pub fn insert(sql_file: String) {
//   let mut client = self::connect();
//
//   let sql = fs::read_to_string(sql_file).unwrap();
//
// let statement = client.prepare("SELECT name FROM people WHERE id = $1");
//
// for id in 0..10 {
//     let rows = client.query(&statement, &[&id]);
//     let name: &str = rows[0].get(0);
//     println!("name: {}", name);
// }
//
//   conn.batch_execute(&sql).unwrap();
// }