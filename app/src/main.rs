pub mod service;
pub mod controller;
pub mod config;
pub mod middleware;

use log::{info, debug};
use sqlx::{MySqlPool, Pool, MySql};
use actix_web::{App, HttpServer, web::Data, middleware::Logger};
use std::{thread, time};

use crate::{config::ApplicationConfig, service::SqlItemService};

#[actix_web::main]
async fn main() {

    let mut db_err = sqlx::Error::ColumnNotFound(String::from(""));
    let mut pool: Option<Pool<MySql>> = None;
    let max_retries: u8 = 3;
    let retry_interval = time::Duration::from_secs(5);

    let config = ApplicationConfig::from_env();

    // Connect to given Database (mariaDB here)
    let connection_uri = format!("mariadb://{}:{}@{}/{}",
        config.user,
        config.password,
        config.host,
        config.db
    );

    debug!("{}", connection_uri.clone());

    for i in 0..max_retries {
        match MySqlPool::connect(connection_uri.as_str()).await {
            Ok(_pool) => {
                info!("Successfully connected to provided database on: [{}]", config.host);
                pool = Some(_pool);
                break;
            },
            // todo: return error etc.
            Err(err) => {
                info!("Could not connect to database, retrying {} times...", max_retries - i);
                thread::sleep(retry_interval);
                db_err = err;
                // panic!("Error while connecting to database: {:?}", err)
            }
        }
    }

    let pool = pool
        .expect(
            format!("Error while connecting to database, {}", db_err.to_string()
        ).as_str());

    SqlItemService::init(&pool.clone(), config.table.clone()).await;

    let port = config.port;
    // Setup Http Server with given controller configuration
    HttpServer::new(move || {
        App::new()
            .app_data( Data::new(pool.clone()) )
            .app_data( Data::new(config.clone()) ) // insert data to be used in the controller
            .wrap(Logger::default())
            .wrap(middleware::SecretCheck)
            .configure(controller::service_config) // setup the controller
    })
    // deadly for docker usage (especially on windows)
    // when the server is bind to 127.0.0.1 instead of 0.0.0.0 it only accepts requests from the localhost
    .bind(("0.0.0.0", port))
    .expect(format!("Can not bind to port: {}", port).as_str())
    .run()
    .await
    .unwrap();
    // fixme: disconnect from database => currently connection is just dropped... not good
    // <server>.handle().stop(true)
}
