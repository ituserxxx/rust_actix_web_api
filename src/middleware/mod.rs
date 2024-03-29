
use md5::Md5;
use md5::Digest;

pub mod auth;

pub fn md5_crypto(password:String) ->String{
    let hashed_password = format!("{:x}", Md5::digest(password.as_bytes()));
    hashed_password
}
pub fn are_strings_equal(input_password: &str, password: &str) -> bool {
    password ==  format!("{:x}", Md5::digest(input_password.as_bytes()))
}

