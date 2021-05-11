extern crate hmac;
pub extern crate rustc_serialize;
extern crate sha2;

use hmac::{Hmac, NewMac};
use jwt::{Header, Token, VerifyWithKey};
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use sha2::Sha384;

use std::collections::BTreeMap;
use std::env;


pub struct ApiKey(pub String);

pub fn read_token(token_str: &str) -> Result<String, String> {
    let secret_key = env::var("APPLICATION_SECRET").expect("APPLICATION_SECRET must be set");

    let key: Hmac<Sha384> = Hmac::new_varkey(secret_key.as_bytes()).unwrap();
    let token: Token<Header, BTreeMap<String, String>, _> = VerifyWithKey::verify_with_key(token_str, &key).unwrap();

    //let header = token.header();
    let claims = token.claims();

    if claims["sub"] == "user" && claims["aud"] == "this_api" {
        Ok("Token OK".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}