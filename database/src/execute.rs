// extern crate postgres;

use postgres::{Client, NoTls, Row, Error};
use postgres::types::ToSql;
use std::fs;

fn connect() -> Client {
  return Client::connect("postgres://wyethjackson@localhost:5432/wyethjackson", NoTls).unwrap();
}

pub fn execute_and_return(sql_file: String, params: &[&(dyn ToSql + Sync)]) -> Vec<Row> {
  let mut client = connect();

  let sql : String = fs::read_to_string(sql_file).unwrap();

  let result : Result<Vec<Row>, Error> = client.query(&sql, params);
  // Checking if the PostgreSQL call failed
  if let Err(err) = result {
    // Handle the error
    println!("PostgreSQL call failed: {}", err);
    return Vec::new();
  }

  return result.unwrap();
}

pub fn execute_no_return(sql_file: String, params: &[&(dyn ToSql + Sync)]) -> bool  {
  let mut client = connect();

  let sql : String = fs::read_to_string(sql_file).unwrap();

  let result : Result<u64, Error> = client.execute(&  sql, params);
  // Checking if the PostgreSQL call failed
  if let Err(err) = result {
    // Handle the error
    println!("PostgreSQL call failed: {}", err);
    return false;
  }
  return true;
}