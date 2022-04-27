use std::future::{ready, Ready};

use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{Error, HttpResponse, web};
use futures_util::future::LocalBoxFuture;

use crate::config::ApplicationConfig;

pub struct SecretCheck;

impl<S, B> Transform<S, ServiceRequest> for SecretCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = SecretCheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecretCheckMiddleware { service }))
    }
}
pub struct SecretCheckMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SecretCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    // actual middleware fn
    fn call(&self, request: ServiceRequest) -> Self::Future {
        let resume_normal = |req| {
            let res = self.service.call(req);
            Box::pin(async move {
                // forwarded responses map to "left" body
                res.await.map(ServiceResponse::map_into_left_body)
            })
        };

        // get secret to compare the request header
        let secret = request.app_data::<web::Data<ApplicationConfig>>()
            .unwrap()
            .secret
            .clone();

        // when no secret is configured, no need to check headers
        match secret {
            Some(secret) => {
                if let Some(header) = request.headers().get("authorized") {
                    if !header.is_empty() {
                        let header = header.to_str().unwrap();
                        if header.eq(&secret) {
                            // resolve and resume as usual
                            return resume_normal(request);
                        }
                    }
                }

                // create error response
                let response = HttpResponse::Unauthorized()
                    .json(serde_json::json!({ "error": "not authorized" }))
                    // constructed responses map to "right" body
                    .map_into_right_body();

                // instead of resume the request -> send Unauthorized response
                Box::pin( async { Ok(ServiceResponse::new(request.into_parts().0, response)) })
            },
            None => {
                // resolve and resume as usual
                resume_normal(request)
            }
        }
    }
}
