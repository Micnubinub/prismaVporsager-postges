//use std::time::SystemTime;
//
//use diesel::PgConnection;
//use diesel::prelude::*;
//use r2d2::{ManageConnection, Pool};
//use r2d2_diesel::ConnectionManager;
//
//use crate::{NewResident, Resident};
//
//use super::super::postgres_schema;
//use super::super::postgres_schema::residents::dsl::*;
//
//pub fn get_resident(res_id: i32, conn: &PgConnection) -> Result<Resident, String> {
//    match residents.find(res_id).first::<Resident>(conn) {
//        Ok(resident) => {
//            Ok(resident)
//        }
//        Err(err) => {
//            Err(err.to_string())
//        }
//    }
//}
//
//pub fn get_resident_email(resident_email: String, conn: &PgConnection) -> Result<Resident, String> {
//    match residents.filter(postgres_schema::residents::email.eq(resident_email)).first::<Resident>(conn) {
//        Ok(resident) => {
//            Ok(resident)
//        }
//        Err(err) => {
//            Err(err.to_string())
//        }
//    }
//}
//
//pub fn delete_resident(res_id: i32, conn: &PgConnection) -> Result<String, String> {
//    match diesel::delete(residents.filter(postgres_schema::residents::id.eq(res_id)))
//        .execute(conn) {
//        Ok(_) => Ok("Successfully deleted".to_string()),
//        Err(err) => Err(err.to_string())
//    }
//}
//
//pub fn insert_resident_user_data(data: Vec<&str>, conn: &PgConnection) -> Result<String, String> {
//    let res_id = match data[0].parse::<i32>() {
//        Ok(i) => i,
//        Err(err) => return Err(err.to_string())
//    };
//
//    let resident = Resident {
//        id: res_id.clone(),
//        first_name: data[1].to_string(),
//        email: data[2].to_string(),
//        dob: data[3].to_string(),
//        checkin: data[4].to_string(),
//        checkout: data[5].to_string(),
//        entry_status: Some(0),
//        profile_photo: Some("".to_string()),
//        last_name: Some("".to_string()),
//    };
//
//    match residents.find(res_id).first::<Resident>(conn) {
//        Ok(resident) => {
//            return update_resident_user_data(resident, conn);
//        }
//        Err(err) => {
//            match diesel::insert_into(residents)
//                .values(&NewResident {
//                    id: resident.id,
//                    first_name: resident.first_name,
//                    last_name: None,
//                    email: resident.email,
//                    dob: resident.dob,
//                    checkin: resident.checkin,
//                    checkout: resident.checkout,
//                }).get_result::<Resident>(conn) {
//                Ok(res) => {
//                    Ok(format!("inserted ID: {}", res.id))
//                }
//                Err(e) => {
//                    Err(format!("failed to get connection from the pool: {}", e.to_string()))
//                }
//            }
//        }
//        Err(e) => {
//            Err(format!("failed to get connection from the pool: {}", e.to_string()))
//        }
//    }
//}
//
//pub fn update_resident_user_data(data: Resident, conn: &PgConnection) -> Result<String, String> {
//    let a = diesel::update(residents.find(&data.id))
//        .set((
//            postgres_schema::residents::dsl::first_name.eq(data.first_name),
//            postgres_schema::residents::dsl::email.eq(data.email),
//            postgres_schema::residents::dsl::dob.eq(data.dob),
//            postgres_schema::residents::dsl::checkin.eq(data.checkin),
//            postgres_schema::residents::dsl::checkout.eq(data.checkout),
//        )).get_result::<Resident>(conn);
//
//    match a {
//        Ok(a) => Ok(format!("res {}", a.id)),
//        Err(e) => {
//            println!("{}", e);
//            Err(e.to_string())
//        }
//    }
//}
