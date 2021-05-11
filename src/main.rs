#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use diesel::PgConnection;
use diesel::result::Error;
use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use rand_core::OsRng;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::{NotFound, Unauthorized};
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};
use sha2::Sha384;

use restful_rusty_rocket::{establish_connection, init_pool};
use restful_rusty_rocket::DbConn;
use restful_rusty_rocket::models::{CreateUser, User, Vehicle};
use restful_rusty_rocket::requests::ApiKey;
use std::collections::BTreeMap;
use std::env;
use std::io;

#[cfg(test)]
mod tests;

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

/// Returns the home page
#[get("/")]
fn hello() -> &'static str {
    "Hello, vehicle world!"
}


#[derive(Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

/// Receives username and password, returns a JWT token on success
#[post("/login", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: DbConn) -> Result<Json<JsonValue>, Unauthorized<String>> {
    let req_username = credentials.username.to_string();
    let req_password = credentials.password.to_string();

    let u = User::find(req_username, &connection);

    if u.len() != 1 {
        Err(Unauthorized(Some(format!("Login failed"))))
    } else {
        let req_password = req_password.as_bytes();
        let parsed_hash = PasswordHash::new(u[0].password.as_str()).unwrap();
        let argon2 = Argon2::default();
        let is_ok = argon2.verify_password(req_password, &parsed_hash).is_ok();

        if is_ok {
            let secret_key = env::var("APPLICATION_SECRET").expect("APPLICATION_SECRET must be set");

            let key: Hmac<Sha384> = Hmac::new_varkey(secret_key.as_bytes()).unwrap();
            let header = Header {
                algorithm: AlgorithmType::Hs384,
                ..Default::default()
            };
            let mut claims = BTreeMap::new();
            claims.insert("sub", "user");
            claims.insert("aud", "this_api");
            let token = Token::new(header, claims).sign_with_key(&key).unwrap();

            let token_string = token.as_str().to_string();

            Ok(Json(json!({ "Token": token_string })))
        } else {
            Err(Unauthorized(Some(format!("Login failed"))))
        }
    }
}

/// - CRUD functions for the vehicle API
/// requests require a valid JWT token

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn vehicle_created(_api_key: ApiKey, vehicle: Vehicle) -> status::Created<Json<Vehicle>> {
    status::Created(
        format!("{host}:{port}/api/vehicle/{id}", host = host(), port = port(), id = vehicle.id).to_string(),
        Some(Json(vehicle)))
}

#[post("/vehicle", format = "application/json", data = "<vehicle>")]
fn create_vehicle(_api_key: ApiKey, vehicle: Json<Vehicle>, connection: DbConn) -> Result<status::Created<Json<Vehicle>>, Status> {
    restful_rusty_rocket::repository::insert(vehicle.into_inner(), &connection)
        .map(|vehicle| vehicle_created(_api_key, vehicle))
        .map_err(|error| error_status(error))
}

#[get("/vehicle")]
fn get_vehicle_list(_api_key: ApiKey, connection: DbConn) -> Result<Json<Vec<Vehicle>>, Status> {
    restful_rusty_rocket::repository::all(&connection)
        .map(|vehicles| Json(vehicles))
        .map_err(|error| error_status(error))
}

#[get("/vehicle/<vehicle_id>")]
fn get_vehicle(_api_key: ApiKey, vehicle_id: i32, connection: DbConn) -> Result<Json<Vehicle>, Status> {
    restful_rusty_rocket::repository::get(vehicle_id, &connection)
        .map(|vehicle| Json(vehicle))
        .map_err(|error| error_status(error))
}

#[put("/vehicle/<vehicle_id>", format = "application/json", data = "<vehicle>")]
fn update_vehicle(_api_key: ApiKey, vehicle_id: i32, vehicle: Json<Vehicle>, connection: DbConn) -> Result<Json<Vehicle>, Status> {
    restful_rusty_rocket::repository::update(vehicle_id, vehicle.into_inner(), &connection)
        .map(|vehicle| Json(vehicle))
        .map_err(|error| error_status(error))
}

#[delete("/vehicle/<vehicle_id>")]
fn delete_vehicle(_api_key: ApiKey, vehicle_id: i32, connection: DbConn) -> Result<Status, Status> {
    match restful_rusty_rocket::repository::get(vehicle_id, &connection) {
        Ok(_) => restful_rusty_rocket::repository::delete(vehicle_id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}

/// - Launch functions

fn create_user_interactive() {
    let con = establish_connection();

    println!("Enter username: ");

    let mut username = String::new();
    match io::stdin().read_line(&mut username) {
        Ok(_) => {
            username.retain(|c| !c.is_whitespace());
            println!("username: {} \nenter password: ", username);
        }
        Err(error) => println!("error: {}", error),
    }

    let mut password = String::new();
    match io::stdin().read_line(&mut password) {
        Ok(_) => {
            password.retain(|c| !c.is_whitespace());
            println!("password entered");
        }
        Err(error) => println!("error: {}", error),
    }
    println!("creating: {}", username);

    let req_password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password_simple(req_password, salt.as_ref()).unwrap().to_string();

    let new_user = CreateUser { username: username, password: password_hash };

    let _u = User::create(new_user, &con);
    println!("User created")
}

fn launch_application() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![hello, login])
        .mount("/api", routes![create_vehicle, get_vehicle_list, get_vehicle,
                update_vehicle, delete_vehicle])
        .launch();
}

/// - Main function

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "launch" => {
            println!("Launching application");
            launch_application()
        }
        "createuser" => {
            println!("Create a new user");
            create_user_interactive()
        }
        _ => println!("No command found, quitting...")
    }
}