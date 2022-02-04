use std::ops::Deref;
use std::sync::Arc;

use actix_web::web::{block, Data, Json, Payload};
use futures::Future;

use crate::actix_web::{HttpRequest, HttpResponse, web};
use crate::AppState;
use crate::model::Query;
use crate::rustc_serialize;

//pub fn msl_hub_login((user, state): (Json<Query>, State<AppState>)) -> FutureResponse<HttpResponse> {
//    state.db.send()
//
//    state.db.send(user.into_inner()).from_err().and_then(|resp| match resp {
//        Ok(msgs) => Ok(HttpResponse::Ok().json(msgs)),
//        Err(_) => Ok(HttpResponse::InternalServerError().json("Internal server error"))
//    }).responder()
//}

//pub fn office_login((user, state): (Json<Query>, State<AppState>)) -> FutureResponse<HttpResponse> {
//    state.db.send(user.into_inner()).from_err().and_then(|resp| match resp {
//        Ok(msgs) => Ok(HttpResponse::Ok().json(msgs)),
//        Err(_) => Ok(HttpResponse::InternalServerError().json("Internal server error"))
//    }).responder()
//}
pub fn db_text(q: String, state: Data<Arc<AppState>>) -> HttpResponse {
    let query: Query = match rustc_serialize::json::decode(q.as_str()) {
        Ok(res) => res,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string())
    };
    db_query(query, state)
}

pub fn db_json(q: Json<Query>, state: Data<Arc<AppState>>) -> HttpResponse {
    db_query(q.into_inner(), state)
}

pub fn db_query(q: Query, state: Data<Arc<AppState>>) -> HttpResponse {
//    let a = match state.actual_db.get() {
//        Ok(pool) => pool,
//        Err(err) => return HttpResponse::InternalServerError().json(err.to_string())
//    };
//    let b = a.deref();
//
//    match handle_req(q, b) {
//        Ok(resp) => resp,
//        Err(resp) => resp
//    }
    HttpResponse::InternalServerError().json(String::from("default response"))
}

