use database::execute;

use postgres::{Client, NoTls, Row};
use postgres::types::ToSql;
use std::fs;

pub fn get_user(id: i32) -> Vec<Row> {
  return execute::execute_and_return(String::from("src/sql/select_user.sql"), &[&id]);
}