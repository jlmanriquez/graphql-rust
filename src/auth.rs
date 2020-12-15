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

use crate::{db::Pool, graphql_schema, jwt, users::models::get_userid_by_username};

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

        if let Some(header) = req.headers().get("Authorization") {
            let token = header.to_str().unwrap().trim_start_matches("Bearer ");

            return if let Ok(username) = jwt::parse_token(token) {
                if let Ok(user_id) = get_userid_by_username(&pool, &username) {
                    let user = graphql_schema::User {
                        id: user_id,
                        name: username,
                    };
                    req.head().extensions_mut().insert(user);
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                } else {
                    Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized()
                                .json(AuthMessage {
                                    message: String::from("wrong user or password"),
                                })
                                .into_body(),
                        ))
                    })
                }
            } else {
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Forbidden()
                            .json(AuthMessage {
                                message: String::from("invalid token"),
                            })
                            .into_body(),
                    ))
                })
            };
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

#[derive(Debug, Serialize)]
struct AuthMessage {
    message: String,
}
