use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use chrono::NaiveDateTime;
use validator::Validate;


#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
  pub id: i32,
  pub name: Option<String>,
  pub email: Option<String>,
  pub updated_at: NaiveDateTime
}

#[derive(Deserialize, PostgresMapper, Serialize, Validate)]
#[pg_mapper(table = "users")] 
pub struct AddUser {
  pub name: Option<String>,
  #[validate(email)]
  pub email: Option<String>,
  pub updated_at: String
}
