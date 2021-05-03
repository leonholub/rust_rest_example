#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use super::schema::vehicles;
use super::models::Vehicle;
use crate::DbConn;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Vehicle>> {
    vehicles::table.load::<Vehicle>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Vehicle> {
    vehicles::table.find(id).get_result::<Vehicle>(connection)
}

pub fn insert(vehicle: Vehicle, connection: &PgConnection) -> QueryResult<Vehicle> {
    diesel::insert_into(vehicles::table)
        .values(&InsertableVehicle::from_vehicle(vehicle))
        .get_result(connection)
}

pub fn update(id: i32, vehicle: Vehicle, connection: &PgConnection) -> QueryResult<Vehicle> {
    diesel::update(vehicles::table.find(id))
        .set(&vehicle)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(vehicles::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "vehicles"]
struct InsertableVehicle {
    name: String,
    coolness: i32,
    wattage: i32,
    description: Option<String>
}

impl InsertableVehicle {
    fn from_vehicle(vehicle: Vehicle) -> InsertableVehicle {
        InsertableVehicle {
            name: vehicle.name,
            coolness: vehicle.coolness,
            wattage: vehicle.wattage,
            description: vehicle.description,
        }
    }
}
