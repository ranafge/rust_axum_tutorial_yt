#![allow(unused)]

use axum::extract::{Path, Query};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));
    // Tcplistener bind with localserver await and unwarp because of futre
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listing from : {}", listener.local_addr().unwrap());
    // axum::serve (listner, app that handle request like get post so on)
    axum::serve(listener, routes_hello).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g `/hello?name=Rana`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    //{:<20} this is string fromater fomat string shift 20 character left  "HANDLER" will place in place holder
    println!("->> {:<20} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g `hello2/Rana`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<20} - handler_hello - {name:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}
