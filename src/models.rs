use super::schema::vehicles;
use serde::{ Serialize, Deserialize };

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: i32,
    pub name: String,
    pub coolness: i32,
    pub wattage: i32,
    pub description: Option<String>
}

//use super::schema::test_data_jsonb;
//
// #[derive(Insertable)]
// #[table_name="test_data_jsonb"]
// pub struct NewPost<'a> {
//     pub name: &'a str,
//     pub dummy_data: &'a str,
// }
