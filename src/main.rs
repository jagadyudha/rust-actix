pub mod config;
pub mod databases;
pub mod handlers;
pub mod models;

use ::config::Config;
use config::default_config::ServerConfig;
use dotenv::dotenv;
use actix_web_validator::JsonConfig;
use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;

use handlers::users::{add_user, get_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ServerConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let json_config = JsonConfig::default().limit(4096)
        .error_handler(|err, _req| {
            config::errors::CustomError::InternalServerError(err.to_string()).into()
        });

    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).app_data(json_config.clone()).service(
            web::resource("/users")
                .route(web::get().to(get_users))
                .route(web::post().to(add_user)),
        )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
