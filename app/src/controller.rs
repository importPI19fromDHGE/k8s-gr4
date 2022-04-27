use log::error;
use actix_web::{get, post, delete, web::{self, Data, ServiceConfig}, Responder, HttpResponse};
use sqlx::{Pool, MySql};

use crate::{config::ApplicationConfig, service::SqlItemService};

macro_rules! await_sql {
    ($e:expr) => {
        match web::block(move || async move { $e.await }).await {
            Ok(result) => match result.await {
                Ok(x) => HttpResponse::Ok().json(x),
                Err(err) => HttpResponse::BadRequest().json(err),
            }
            Err(e) => {
                error!("Error while getting, {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    };
}

//
// in the following fn's, parameters with Data<> can be use due to the configuration of the application (see main)
//

#[get("/")]
/// get all entries => /
async fn get_all( pool: Data<Pool<MySql>>, cfg: Data<ApplicationConfig> ) -> impl Responder {
    await_sql!(SqlItemService::get_items(&pool, cfg.table.clone()))
}

#[post("/")]
/// insert by given path id with provided body => /
/// requires {"content":"<text>"}, provided id will get ignored but isn't forbidden
async fn insert_todo( pool: Data<Pool<MySql>>, cfg: Data<ApplicationConfig>,
    item: web::Json<crate::service::Item>
) -> impl Responder {
    await_sql!(SqlItemService::add_item(&pool, cfg.table.clone(), item.content.clone()))
}

#[get("/{id}")]
/// get item by given path id => /{id}
async fn get_by_id( pool: Data<Pool<MySql>>, cfg: Data<ApplicationConfig>,
    id: web::Path<String>
) -> impl Responder {
    await_sql!(SqlItemService::get_item_by_id(&pool, cfg.table.clone(), id.to_string()))
}

#[delete("/{id}")]
/// delete by given path id => /{id}
async fn delete_by_id( pool: Data<Pool<MySql>>, cfg: Data<ApplicationConfig>,
    id: web::Path<String>
) -> impl Responder {
    await_sql!(SqlItemService::delete_item_by_id(&pool, cfg.table.clone(), id.to_string()))
}

pub fn service_config( cfg: &mut ServiceConfig ) {
    cfg
        .service(get_all)
        .service(insert_todo)
        .service(get_by_id)
        .service(delete_by_id);
}
