use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web::dev::ServiceRequest;
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::{
    tools::jwt,
};

// pub async fn jwt_middleware(req: ServiceRequest, _: web::Data<actix_web::App>) -> Result<ServiceRequest, Error> {
//     let token = req.headers().get("Authorization").and_then(|value| value.to_str().ok());
//
//     match token {
//         Some(token) => {
//             let token_data = decode::<jwt::Claims>(
//                 &token,
//                 &DecodingKey::from_secret("secret".as_ref()),
//                 &Validation::default()
//             );
//             match token_data {
//                 Ok(_) => Ok(req),
//                 Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
//             }
//         },
//         None => Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
//     }
// }