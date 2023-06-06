extern crate postgres;

use postgres::{Client, NoTls};
use std::fs;

fn connect() -> Client {
  return Client::connect("postgres://wyethjackson@localhost:5432/wyethjackson", NoTls).unwrap();
}

pub fn select(sql_file: String) {
  let mut conn = self::connect();

  let sql = fs::read_to_string(sql_file).unwrap();

  conn.batch_execute(&sql).unwrap();
}

pub fn insert(sql_file: String) {
  let mut client = self::connect();

  let sql = fs::read_to_string(sql_file).unwrap();

let statement = client.prepare("SELECT name FROM people WHERE id = $1");

for id in 0..10 {
    let rows = client.query(&statement, &[&id]);
    let name: &str = rows[0].get(0);
    println!("name: {}", name);
}

  conn.batch_execute(&sql).unwrap();
}