mod models;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::prelude::*;
use models::Category;
use models::Product;
use serde_json::json;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::main;
use uuid::Uuid;

#[derive(Clone)]

struct AppState {
    categories: Arc<Mutex<Vec<Category>>>,
    products: Arc<Mutex<Vec<Product>>>,
}

#[main]
async fn main() {
    // initialize the shared state
    let shared_state = AppState {
        categories: Arc::new(Mutex::new(Vec::new())),
        products: Arc::new(Mutex::new(Vec::new())),
    };

    // configure routing
    let app = Router::new()
        .route("/products", get(list_products).post(create_product))
        .route(
            "/product/:id",
            get(get_product).put(update_product).delete(delete_product),
        )
        .route("/category", get(get_categories).post(create_category))
        .route(
            "/category/:id",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
        .with_state(shared_state);

    // run our application
    let addr = SocketAddr::from((([127, 0, 0, 1], 5000)));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_products(State(state): State<AppState>) -> impl IntoResponse {
    let products = state.products.lock().unwrap();
    Json(products.clone());
}

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<Product>,
) -> impl IntoResponse {
    let mut products = state.products.lock().unwrap();
    let local: DateTime<Utc> = Utc::now();
    let new_product = Product {
        id: Uuid::new_v4(),
        name: payload.name,
        price: payload.price,
        category: payload.category,
        created_at: local,
        quantity: payload.quantity,
        updated_at: local,
    };

    products.push(new_product.clone());
    (StatusCode::CREATED, Json(new_product))
}

async fn get_product(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let products = state.products.lock().unwrap();

    if let Some(product) = products.iter().find(|&product| product.id == id) {
        Json(product.clone()).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "product not found" })),
        )
            .into_response()
    }
}

async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Product>,
) -> impl IntoResponse {
    let mut products = state.products.lock().unwrap();
    if let Some(product) = products.iter_mut().find(|product| product.id == id) {
        product.name = payload.name;
        product.quantity = payload.quantity;
        product.price = payload.price;
    }
}

async fn delete_product(State(state): State<AppState>, Path(id): Path<Uuid>) {
    let mut products = state.products.lock().unwrap();
    if let Some(pos) = products.iter().position(|product| product.id == id) {
        products.remove(pos);

        (
            StatusCode::NO_CONTENT,
            Json(json!({ "error": "no content was found" })),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "product was not found" })),
        )
            .into_response()
    }
}

async fn get_categories() {}

async fn create_category() {}

async fn get_category() {}

async fn update_category() {}

async fn delete_category() {}
