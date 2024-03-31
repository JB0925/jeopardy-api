#[macro_use] extern crate rocket;
extern crate serde_json;
use once_cell::sync::Lazy;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Method;
use rocket::http::Status;
use rocket::Request;
use rocket::Response;
use rocket::response::{self, Responder};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde_json::Value;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Once;

use crate::data::Categories;
use crate::data::CategoryDetails;
use crate::constants::constants::{CATEGORY_IDS, TOTAL_CATEGORIES};

mod data;
mod constants;

static CATEGORIES: Lazy<Mutex<Categories>> = Lazy::new(|| Mutex::new(Categories::new()));
static CATEGORY_DETAILS: Lazy<Mutex<CategoryDetails>> = Lazy::new(|| Mutex::new(CategoryDetails::new()));
static INIT: Once = Once::new();

pub fn initialize_logger() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

#[derive(Serialize, Deserialize)]
pub struct CategoriesResponse {
    categories: Vec<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryDetailsResponse {
    details: HashMap<String, Value>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

enum ApiCategoriesResponse {
    Success(Json<CategoriesResponse>),
    Error(Status, Json<ErrorResponse>),
}

enum ApiCategoryDetailsResponse {
    Success(Json<CategoryDetailsResponse>),
    Error(Status, Json<ErrorResponse>),
}

impl<'r> Responder<'r, 'static> for ApiCategoriesResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiCategoriesResponse::Success(json) => json.respond_to(request),
            ApiCategoriesResponse::Error(status, json) => Response::build_from(json.respond_to(request).unwrap())
                .status(status)
                .ok(),
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiCategoryDetailsResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiCategoryDetailsResponse::Success(json) => json.respond_to(request),
            ApiCategoryDetailsResponse::Error(status, json) => Response::build_from(json.respond_to(request).unwrap())
                .status(status)
                .ok(),
        }
    }
}

fn is_valid_count(count: i32) -> bool {
    count > 0 && count <= TOTAL_CATEGORIES
}

fn is_valid_id(index: &str) -> bool {
    return CATEGORY_IDS.contains(&index);
}

#[get("/api/categories?<count>")]
fn get_categories(count: Option<i32>) -> ApiCategoriesResponse {
    let count = count.unwrap_or(TOTAL_CATEGORIES);
    if is_valid_count(count) {
        log::info!("main.rs::get_categories - Getting {} categories", count);
        let categories = CATEGORIES.lock().unwrap();
        let categories = categories.get_categories(count);
        log::info!("main.rs::get_categories - Found {} categories", categories.len());
        ApiCategoriesResponse::Success(Json(CategoriesResponse { categories }))
    } else {
        log::error!("main.rs::get_categories - Invalid count: {}", count);
        let error_message = format!("main.rs::get_categories - Invalid count. Count must be between 1 and {}; got {}", TOTAL_CATEGORIES, count);
        let error_response = ErrorResponse { error: error_message };
        ApiCategoriesResponse::Error(Status::BadRequest, Json(error_response))
    }
}

#[get("/api/categories/<id>")]
fn get_category(id: i32) -> ApiCategoriesResponse {
    if is_valid_id(id.to_string().as_str()) {
        log::info!("main.rs::get_category - Getting category at id {}", id);
        let categories = CATEGORIES.lock().unwrap();
        let category = categories.get_category(id);
        ApiCategoriesResponse::Success(Json(CategoriesResponse { categories: vec![category.clone()] }))
    } else {
        log::error!("main.rs::get_category - Invalid id: {}", id);
        let error_message = format!("Invalid id. ID must be in {:?}; got {}", CATEGORY_IDS, id);
        let error_response = ErrorResponse { error: error_message };
        ApiCategoriesResponse::Error(Status::BadRequest, Json(error_response))
    }
}

#[get("/api/details")]
fn get_category_details() -> ApiCategoryDetailsResponse {
    log::info!("main.rs::get_category_details - Getting category details");
    let details = CATEGORY_DETAILS.lock().unwrap();
    let details = details.get_details();
    ApiCategoryDetailsResponse::Success(Json(CategoryDetailsResponse { details: details.clone() }))
}

#[get("/api/details/<category_number>")]
fn get_category_detail(category_number: &str) -> ApiCategoryDetailsResponse {
    if CATEGORY_IDS.contains(&category_number) {
        log::info!("main.rs::get_category_detail - Getting detail for category {}", category_number);
        let details = CATEGORY_DETAILS.lock().unwrap();
        let detail = details.get_detail(category_number);
        let mut details_map = HashMap::new();
        details_map.insert(category_number.to_string(), detail.clone());
        ApiCategoryDetailsResponse::Success(Json(CategoryDetailsResponse { details: details_map }))
    } else {
        log::error!("main.rs::get_category_detail - Invalid category number: {}", category_number);
        let error_message = format!("Invalid category number. Category number must be one of {:?}", CATEGORY_IDS);
        let error_response = ErrorResponse { error: error_message };
        ApiCategoryDetailsResponse::Error(Status::BadRequest, Json(error_response))
    }
}

#[launch]
fn rocket() -> _ {
    initialize_logger();
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
        get_categories, get_category, get_category_details, get_category_detail
    ])
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;

    #[test]
    fn test_get_categories() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/categories").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("categories"));
    }

    #[test]
    fn test_get_categories_with_count() {
        let count = 5;
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(format!("/api/categories?count={}", count)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().expect("Failed to read response body");
        assert!(body.contains("categories"));
        let parsed_response: CategoriesResponse = serde_json::from_str(&body).expect("Failed to parse JSON");
        assert_eq!(parsed_response.categories.len(), count as usize);
    }

    #[test]
    fn test_get_category() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/categories/2").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_invalid_count() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/categories?count=0").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_invalid_id() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/categories/25").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_get_category_details() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/details").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("details"));
    }

    #[test]
    fn test_get_category_detail() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/details/2").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_invalid_category_number() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/details/1").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }
}