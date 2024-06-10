mod models;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
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
        .route("/products", get(list_products).post(create_products))
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



 