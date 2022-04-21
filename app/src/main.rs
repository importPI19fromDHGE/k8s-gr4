pub mod service;
pub mod controller;
pub mod config;
pub mod middleware;

use log::{info, debug};
use dotenv::dotenv;
use sqlx::MySqlPool;
use actix_web::{App, HttpServer, web::Data, middleware::Logger};

use crate::config::ApplicationConfig;

#[actix_web::main]
async fn main() {
    // setup logger and dotenv
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().filter_or("K8SGR4_LOG","info"));

    let config = ApplicationConfig::from_env();

    // Connect to given Database (mariaDB here)
    let connection_uri = format!("mariadb://{}:{}@{}/{}",
        config.user,
        config.password,
        config.host,
        config.db
    );

    debug!("{}", connection_uri.clone());

    let pool = match MySqlPool::connect(connection_uri.as_str()).await {
        Ok(pool) => {
            info!("Successful connected to provide database on: [{}]", config.host);
            pool
        },
        Err(err) => {
            panic!("Error while connecting to database: {:?}", err)
        }
    };

    // Setup Http Server with given controller configuration
    let port = config.port;
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
