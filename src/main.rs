use actix_web::{App,web, HttpServer};
mod api;
mod controller;
mod middleware;
mod tools;
use crate::{
    controller::hello,
    middleware::auth,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/hello")
                .route("/", web::get().to(controller::hello::hello))
                .route("/echo", web::post().to(controller::hello::echo))
                .route("/aaa/{name}", web::get().to(controller::hello::aaa))
                .route("/del/{id}", web::delete().to(controller::hello::del))
                .route("/patch", web::patch().to(controller::hello::patch))
        )
        .service(
            web::resource("/info")
            .wrap(from_fn(my_mw))
            .route(web::post().to(controller::hello::info))
        )
    })
    .bind(("127.0.0.1", 8877))?
    .run()
    .await
}

use actix_web::{
     Error,
    dev::{ServiceRequest, ServiceResponse, Service as _},
};
use actix_web_lab::middleware::Next;
use actix_web_lab::middleware::from_fn;
async fn my_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    next.call(req).await
    // post-processing
}

