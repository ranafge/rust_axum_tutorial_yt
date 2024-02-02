#![allow(unused)]

use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;



#[tokio::main]
async fn main() {
    
    let routes_hello = Router::new().route("/hello", get(|| async {
        Html("Hello <strong>World!</strong>")
    }));
    // Tcplistener bind with localserver await and unwarp because of futre
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listing from : {}", listener.local_addr().unwrap());
    // axum::serve (listner, app that handle request like get post so on)
    axum::serve(listener, routes_hello)
    .await
    .unwrap();
}