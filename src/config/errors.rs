use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use serde_json::json;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Display, From, Debug)]
pub enum CustomError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
    InternalServerError,
}
impl std::error::Error for CustomError {}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::NotFound => {
              HttpResponse::NotFound().finish()
            },
            _ => {
                println!("{}", self.to_string());
                HttpResponse::InternalServerError().json(json!({ "error":"something went wrong" }))
            }
        }
    }
}