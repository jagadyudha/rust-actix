use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::{config::errors::CustomError, models};
use chrono::NaiveDateTime;
use models::users::{User, AddUser};

pub async fn get_users(client: &Client) -> Result<Vec<User>, CustomError> {
    let stmt = "SELECT * FROM users";
    let stmt = client.prepare(&stmt).await.unwrap();
    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();
    Ok(results)
}

pub async fn add_user(client: &Client, user_data: AddUser) -> Result<Vec<User>, CustomError> {
    let stmt = "INSERT INTO users (email, name, updated_at) VALUES ($1, $2, $3) RETURNING *";
    let stmt = client.prepare(&stmt).await.unwrap();
    let updated_at = NaiveDateTime::parse_from_str(&user_data.updated_at, "%Y-%m-%dT%H:%M:%S%.fZ")
    .expect("Failed to parse ISO string");

    let results = client
        .query(&stmt, &[
            &user_data.email,
            &user_data.name,
            &updated_at
        ])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();
    
    Ok(results)
}