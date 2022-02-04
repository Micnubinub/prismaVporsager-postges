extern crate image;

use crate::actix_web::{HttpRequest, HttpResponse, web};
use crate::{rustc_serialize, main};
use std::ops::Deref;
use std::sync::Arc;
use crate::AppState;
use actix_web::web::{block, Data, Json, Payload};
use futures::Future;
use std::time::{Duration, Instant};
use image::GenericImageView;
use image::imageops;

#[derive(Debug, Deserialize, Serialize, RustcDecodable, RustcEncodable)]
pub struct ImageResizeRequest {
    pub input_path: String,
    pub output_path: String,
    pub w: u32,
    pub h: u32,
    pub keep_scale: Option<bool>,
}

pub fn resize_image_request(q: Json<ImageResizeRequest>, state: Data<Arc<AppState>>) -> HttpResponse {
    let request = q.into_inner();
    resize_image(request.input_path, request.output_path, request.w, request.h, match request.keep_scale {
        None => true,
        Some(b) => b
    })
}

pub fn resize_image(input_path: String, output_path: String, mut w: u32, mut h: u32, keep_scale: bool) -> HttpResponse {
    let start = Instant::now();
    let mut img = match image::open(input_path) {
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
        Ok(img) => img
    };

    let old_w = img.dimensions().0;
    let old_h = img.dimensions().1;

    if keep_scale {
        if old_w > old_h {
            h = ((old_h as f64) / (old_w as f64) * (h as f64)) as u32;
        } else {
            w = ((old_w as f64) / (old_h as f64) * (w as f64)) as u32;
        }
    }

    println!("w: {}, h: {}", w, h);
    img = img.resize(w, h, imageops::Gaussian);
    let resp = match img.save(&output_path) {
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
        Ok(img) => {
            HttpResponse::InternalServerError().json(String::from(output_path))
        }
    };

    println!("lapsed: {}", start.elapsed().as_millis());
    resp
}
