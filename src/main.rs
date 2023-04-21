use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info};

mod db;
mod errors;
mod models;
mod routes;
mod schema;

use crate::{
    errors::CustomError,
    routes::{
        category::*, city::*, favorite::*, location::*, reservation::*, restaurant::*, shirin,
        table::*, user::*,
    },
};

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), CustomError> {
    dotenv().ok();

    db::init()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/images", "./images/").show_files_listing())
            .service(cities)
            .service(restaurants)
            .service(shirin)
            .service(register)
            .service(login_service)
            .service(categories)
            .service(locations)
            .service(get_favorites)
            .service(add_favorite)
            .service(delete_favorite)
            .service(get_tables)
            .service(get_reservations)
            .service(add_reservation)
            .service(delete_reservation)
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await?;

    Ok(())
}
