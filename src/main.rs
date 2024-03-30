#[macro_use] extern crate rocket;
extern crate serde_json;
use once_cell::sync::Lazy;
use rocket::outcome::Outcome;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::form::FromForm;
use rocket::http::Method;
use rocket::http::Status;
use rocket::Request;
use rocket::Response;
use rocket::request::{self, FromRequest};
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde_json::Value;

use std::collections::HashMap;
use std::sync::Mutex;

use crate::data::Categories;
use crate::constants::constants::TOTAL_CATEGORIES;

mod data;
mod constants;

static CATEGORIES: Lazy<Mutex<Categories>> = Lazy::new(|| Mutex::new(Categories::new()));

#[derive(Serialize)]
pub struct CategoriesResponse {
    categories: Vec<HashMap<String, Value>>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

enum ApiResponse {
    Success(Json<CategoriesResponse>),
    Error(Status, Json<ErrorResponse>),
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiResponse::Success(json) => json.respond_to(request),
            ApiResponse::Error(status, json) => Response::build_from(json.respond_to(request).unwrap())
                .status(status)
                .ok(),
        }
    }
}

fn is_valid(count: i32) -> bool {
    count > 0 && count <= TOTAL_CATEGORIES
}

#[get("/api/categories?<count>")]
fn get_categories(count: Option<i32>) -> ApiResponse {
    let count = count.unwrap_or(TOTAL_CATEGORIES);
    if is_valid(count) {
        let categories = CATEGORIES.lock().unwrap();
        let categories = categories.get_categories(count);
        ApiResponse::Success(Json(CategoriesResponse { categories }))
    } else {
        let error_message = format!("Invalid count. Count must be between 1 and {}; got {}", TOTAL_CATEGORIES, count);
        let error_response = ErrorResponse { error: error_message };
        ApiResponse::Error(Status::BadRequest, Json(error_response))
    }
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::build().attach(cors.to_cors().unwrap()).mount("/", routes![
        get_categories
    ])
}