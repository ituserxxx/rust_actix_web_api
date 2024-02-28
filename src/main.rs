mod api;
mod controller;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(controller::hello::hello)
            .service(controller::hello::echo)
            .service(controller::hello::aaa)
            .service(controller::hello::del)
            .service(controller::hello::patch)
            .service(controller::hello::info)

    })
    .bind(("127.0.0.1", 8877))?
    .run()
    .await
}