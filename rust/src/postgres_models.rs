use diesel;
use diesel::prelude::*;

use super::postgres_schema::*;

#[derive(Serialize, RustcDecodable, RustcEncodable)]
pub struct SerializableString {
    pub data: String,
}

#[derive(Insertable, RustcDecodable, RustcEncodable)]
#[table_name = "residents"]
pub struct NewResident {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub dob: String,
    pub checkin: String,
    pub checkout: String,
    pub email: String,
}

#[derive(Debug, Queryable, Deserialize, Associations, Identifiable, Serialize, RustcDecodable, RustcEncodable)]
#[table_name = "residents"]
pub struct Resident {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub dob: String,
    pub checkin: String,
    pub checkout: String,
    pub email: String,
    pub profile_photo: Option<String>,
    pub entry_status: Option<i32>,
}
