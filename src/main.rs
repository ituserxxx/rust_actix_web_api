use actix_web::{App,web, HttpServer};

mod api;
mod controller;
mod middleware;
mod tools;
use crate::{
    controller::hello,
//     middleware::auth,
};



use actix_web_lab::middleware::Next;
use actix_web::{
     Error,
      body::MessageBody,
    dev::{ServiceRequest, ServiceResponse, Service as _},
};
use actix_web_lab::middleware::from_fn;

async fn midd_befor(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
        println!("1111-before-processing {}", req.path());
        if let Some(token) = req.headers().get("Authorization") {
            //  校验失败，返回未认证的错误
            if token != "valid_token" {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            }
        }
    //   next.call(req).await
        // 调用 next 处理请求
        let response = next.call(req).await;
        match &response {
            Ok(resp) => {
                println!("1111-after-processing {}", resp.request().path());

            }
            Err(_) => {
                println!("1111-error-processing");
                 return Err(actix_web::error::ErrorBadRequest("ErrorBadRequest"))
            }
        }
        response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/info")
                .wrap(from_fn(midd_befor))
                .route(web::post().to(hello::info))
        )
        .service(
            web::scope("/hello")
                .route("/", web::get().to(hello::hello))
                .route("/post_t", web::post().to(hello::post_t))
                .route("/get_t/{name}", web::get().to(hello::get_t))
                .route("/del_t/{id}", web::delete().to(hello::del_t))
                .route("/patch_t", web::patch().to(hello::patch_t)),
        )

    })
    .bind(("127.0.0.1", 8877))?
    .run()
    .await
}










