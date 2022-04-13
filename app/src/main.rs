pub mod service;
pub mod controller;

use std::env;

use log::{info, debug};
use dotenv::dotenv;
use sqlx::MySqlPool;
use actix_web::{App, HttpServer, web::{self, Data}};

use crate::service::ItemService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logger and dotenv
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().filter_or("K8SGR4_LOG","info"));

    ///////////////////////////////
    // CONFIG SETUP
    ///////////////////////////////
    // required
    let env_var_usr     = "MARIADB_USER";
    let env_var_pwd     = "MARIADB_PASSWORD";
    let env_var_host    = "MARIADB_HOST";
    let env_var_db      = "MARIADB_DATABASE";
    let env_var_table   = "MARIADB_TABLE";
    // optional => has default
    let env_var_port    = "K8SGR4_PORT";

    // func to get var or panic with error when environment variable wasn't found
    let get_env_var = | env_var: &str |  env::var(env_var)
        .expect(format!("Couldn't get environment variable for [{}]", env_var).as_str());

    let user     = get_env_var(env_var_usr);
    let password = get_env_var(env_var_pwd);
    let host     = get_env_var(env_var_host);
    let database = get_env_var(env_var_db);
    let table    = get_env_var(env_var_table);
    info!("Found all required environment variable");
    info!("Will use: User: [{}], Host: [{:?}], DB: [{}], Table: [{}]", user, host, database, table);

    ///////////////////////////////
    // Connect to given Database
    ///////////////////////////////
    let connection_uri = format!("mariadb://{}:{}@{}/{}", user, password, host, database);
    debug!("{}", connection_uri.clone());

    let pool = MySqlPool::connect(connection_uri.as_str())
        .await
        .expect("Error while connecting to database");

    ///////////////////////////////
    // ACTUAL REST HANDLING
    ///////////////////////////////
    HttpServer::new(move || {
        // setup service which provides pool to routes later
        let service = ItemService::new(pool.clone(), table.clone());
        App::new()
            // provides data which can be used in each route => service which contains pool and table
            .app_data(Data::new(service) )
            // route setup for root "/"
            .service(web::resource("/")
                // GET on root retrieves all entries
                .route(web::get().to(controller::get_all))
                // POST to root inserts the provided json (probably has to be escaped otherwise it just failed)
                .route(web::post().to(controller::insert_todo))
            )
            // route setup for root "/" with id as path
            .service(web::resource("/{id}")
                // GET on root with an provided id returns the item with that id
                .route(web::get().to(controller::get_by_id))
                // DELETE on root with an provided id deletes the item with that id
                .route(web::delete().to(controller::delete_by_id))
            )
    })
    .bind(format!("127.0.0.1:{}", env::var(env_var_port).unwrap_or("8080".to_string())))?
    .run()
    .await
}
