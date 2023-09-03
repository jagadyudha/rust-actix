use crate::{models, databases, config::errors::CustomError};

use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use models::users::AddUser;
use databases as db;

pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(CustomError::PoolError)?;
    let users = match db::users::get_users(&client).await {
        Ok(users) => users,
        Err(error) => {
            return Ok(
                HttpResponse::InternalServerError().json(format!("Database error: {}", error))
            )
        }
    };
    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: actix_web_validator::Json<AddUser>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    if user.updated_at.is_empty() {
         HttpResponse::BadRequest().json("Username is missing");
    }
    let client: Client = db_pool.get().await.map_err(CustomError::PoolError)?;
    let user_data: AddUser = user.into_inner();
    let users = db::users::add_user(&client, user_data).await?;
    Ok(HttpResponse::Ok().json(users))
}
