#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;

#[cfg(test)]
mod tests;

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
#[post("/vehicle")]
fn create_vehicle() -> &'static str { "Vehicle created" }

#[get("/vehicle")]
fn get_vehicle_list() -> &'static str { "Vehicle list" }

#[get("/vehicle/<id>")]
fn get_vehicle(id: &RawStr) -> String {
    format!("Looking for {}...", id)
}

#[put("/vehicle/<id>")]
fn update_vehicle(id: &RawStr) -> String {
    format!("Updating {}...", id)
}

#[delete("/vehicle/<id>")]
fn delete_vehicle(id: &RawStr) -> String {
    format!("Deleting {}...", id)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, login])
        .mount("/api", routes![create_vehicle, get_vehicle_list, get_vehicle,
        update_vehicle, delete_vehicle])
        .launch();
}
