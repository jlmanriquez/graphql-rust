use std::pin::Pin;

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use serde::Serialize;

use crate::{db::Pool, jwt, users::models::get_userid_by_username};

pub struct Auth {}

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let pool = req
            .app_data::<actix_web::web::Data<Pool>>()
            .unwrap()
            .get()
            .unwrap();

        let mut authenticated_err: Option<String> = None;
        // Siguiendo el ejemplo de https://www.howtographql.com/ en caso de no venir
        // el header de autenticacion se deja pasar.
        if let Some(header) = req.headers().get("Authorization") {
            let bearer = header.to_str().unwrap();
            let token = bearer.trim_start_matches("Bearer ");

            if let Ok(username) = jwt::parse_token(token) {
                match get_userid_by_username(&pool, &username) {
                    Ok(_) => {}
                    _ => authenticated_err = Some(String::from("wrong user or password")),
                }
            } else {
                authenticated_err = Some(String::from("invalid token"));
            }
        }

        match authenticated_err {
            Some(err_msg) => Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(AuthMessage { message: err_msg })
                        .into_body(),
                ))
            }),
            None => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct AuthMessage {
    message: String,
}
