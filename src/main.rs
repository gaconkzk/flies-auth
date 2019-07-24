// main.rs
#![allow(dead_code)] // This is very useful in dev mode

#[macro_use] // we need extern because we need macro
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod utils;
mod schema;
mod errors;
mod invitation;
mod email;

fn main() -> std::io::Result<()> {
    // TODO env management
    dotenv::dotenv().ok();
    // RUST_LOG for logger level
    env_logger::init();
    let database_url = std::env::var("FA_DATABASE_URL").expect("FA_DATABASE_URL must be set");

    // DB Connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Sorry, we can't create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "8088".to_string());

    // Start the http server
    HttpServer::new(move || {
        App::new()
          // data from pool
          .data(pool.clone())
          // enable logger
          .wrap(middleware::Logger::default())
          .wrap(IdentityService::new(
            CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
              .name("auth")
              .path("/")
              .domain(domain.as_str())
              .max_age_time(chrono::Duration::days(1))
              .secure(false), // this can only be true if we have https
          ))
          // limit maximum amount of data that server will accept
          .data(web::JsonConfig::default().limit(8048))
          .service(
            web::scope("/api")
              // .service(
              //   web::resource("/invitation")
              //     .route(web::post().to_async(invitation::handler::post_invitation)),
              // )
              .service(
                web::resource("/register/{invitation_id}")
                  .route(web::post().to(||{})),
              )
              .service(
                web::resource("/auth")
                  .route(web::post().to(||{}))
                  .route(web::delete().to(||{}))
                  .route(web::get().to(||{})),
              ),
          )
          .service(web::resource("/").route(web::get().to(|| {
            HttpResponse::Ok()
              .body("Hello actix!")
          })))
    })
    .bind(format!("127.0.0.1:{}",port))?
    .run()
}
