use actix_web::{web, Responder, HttpResponse};
use log::error;

macro_rules! resolve_result {
    ( $e:expr ) => {
        match $e {
            Ok(x) => HttpResponse::Ok().json(x),
            Err(err) => HttpResponse::BadRequest().json(err),
        }
    }
}

/// get all entries => /
pub async fn get_all(service: web::Data<crate::ItemService>) -> impl Responder {
    match web::block(move || async move { service.get_items().await }).await {
        Ok(result) => resolve_result!(result.await),
        Err(e) => {
            error!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// insert by given path id with provided body => /
/// requires {"content":"<text>"}, if id is provided it will get ignored
pub async fn insert_todo(service: web::Data<crate::ItemService>, item: web::Json<crate::service::Item>) -> impl Responder {
    match web::block(move || async move { service.add_item(item.content.clone()).await}).await {
        Ok(result) => resolve_result!(result.await),
        Err(e) => {
            error!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// get item by given path id => /{id}
pub async fn get_by_id(service: web::Data<crate::ItemService>, id: web::Path<String>) -> impl Responder {
    match web::block(move || async move { service.get_item_by_id(id.to_string()).await }).await {
        Ok(result) => resolve_result!(result.await),
        Err(e) => {
            error!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// delete by given path id => /{id}
pub async fn delete_by_id(service: web::Data<crate::ItemService>, id: web::Path<String>) -> impl Responder {
    match web::block(move || async move { service.delete_item_by_id(id.to_string()).await}).await {
        Ok(result) => resolve_result!(result.await),
        Err(e) => {
            error!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
