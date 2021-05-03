#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::env;

// use self::diesel::prelude::*;

use diesel::result::Error;
use rocket::http::RawStr;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use restful_rusty_rocket::schema::*;
use restful_rusty_rocket::DbConn;
use restful_rusty_rocket::models::Vehicle;
use restful_rusty_rocket::init_pool;

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

/// Receives username and password, returns a JWT token on success
#[post("/login")]
fn login() -> &'static str {
    "User login"
}

/// CRUD functions for the vehicle API
/// requests require a valid JWT token
///

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn vehicle_created(vehicle: Vehicle) -> status::Created<Json<Vehicle>> {
    status::Created(
        format!("{host}:{port}/api/vehicle/{id}", host = host(), port = port(), id = vehicle.id).to_string(),
        Some(Json(vehicle)))
}

#[post("/vehicle", format="application/json", data="<vehicle>")]
fn create_vehicle(vehicle: Json<Vehicle>, connection: DbConn) -> Result<status::Created<Json<Vehicle>>, Status> {
    restful_rusty_rocket::repository::insert(vehicle.into_inner(), &connection)
        .map(|vehicle| vehicle_created(vehicle))
        .map_err(|error| error_status(error))
}

#[get("/vehicle")]
fn get_vehicle_list(connection: DbConn) -> Result<Json<Vec<Vehicle>>, Status> {
    restful_rusty_rocket::repository::all(&connection)
        .map(|vehicles| Json(vehicles))
        .map_err(|error| error_status(error))
}

#[get("/vehicle/<vehicle_id>")]
fn get_vehicle(vehicle_id: i32, connection: DbConn) -> Result<Json<Vehicle>, Status> {
    restful_rusty_rocket::repository::get(vehicle_id, &connection)
        .map(|vehicle| Json(vehicle))
        .map_err(|error| error_status(error))
}

#[put("/vehicle/<vehicle_id>", format = "application/json", data = "<vehicle>")]
fn update_vehicle(vehicle_id: i32, vehicle: Json<Vehicle>, connection: DbConn) -> Result<Json<Vehicle>, Status> {
    restful_rusty_rocket::repository::update(vehicle_id, vehicle.into_inner(), &connection)
        .map(|vehicle| Json(vehicle))
        .map_err(|error| error_status(error))
}

#[delete("/vehicle/<vehicle_id>")]
fn delete_vehicle(vehicle_id: i32, connection: DbConn)-> Result<Status, Status> {

    match restful_rusty_rocket::repository::get(vehicle_id, &connection) {
        Ok(_) => restful_rusty_rocket::repository::delete(vehicle_id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![hello, login])
        .mount("/api", routes![create_vehicle, get_vehicle_list, get_vehicle,
        update_vehicle, delete_vehicle])
        .launch();
}
