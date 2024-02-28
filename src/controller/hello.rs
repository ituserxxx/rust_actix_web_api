use actix_web::{get, post,delete,patch, web,  HttpResponse,  Responder,Result};
use serde::Serialize;
use serde::Deserialize;
//// curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8877/


// curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8877/
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// curl -X POST -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8877/echo
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[derive(Serialize)]
struct MyObj2 {
    id: i64,
}

// curl -X DELETE -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8877/del/888
#[delete("/del/{id}")]
async fn del(id: web::Path<i64>) -> Result<impl Responder> {
        let obj = MyObj2 {
                id: *id,
            };
            Ok(web::Json(obj))
}

#[derive(Deserialize,Serialize)]
struct Info {
    username: String,
}


// curl -X PATCH -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8877/patch
#[patch("/patch")]
async fn patch(a: web::Json<Info>) ->  Result<impl Responder>  {
//     Ok(format!("Welcome {}!", info.username))
     Ok(web::Json(a))
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

// curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"id":1111}' http://127.0.0.1:8877/aaa/xxx1
#[get("/aaa/{name}")]
async fn aaa(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}


// curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"username":"yuiyui"}' http://127.0.0.1:8877/info

#[get("/info")]
async fn info(info: web::Json<Info>) ->  Result<impl Responder>  {
//     Ok(format!("Welcome {}!", info.username))
     Ok(web::Json(info))
}