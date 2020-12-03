#[macro_use]
extern crate diesel;

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use context::GraphQLContext;
use db::Pool;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::graphql_schema::{create_schema, Schema};

mod graphql_schema;
mod schema;
mod users;
mod db;
mod links;
mod context;
mod jwt;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    pool: web::Data<Pool>,
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };

    let user = web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = std::sync::Arc::new(create_schema());

    let url_db = std::env::var("DATABASE_URL").expect("DB url not found");
    let pool = db::get_pool(url_db.as_str());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
