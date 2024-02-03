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
    age: Option<i32>
}

// e.g `/hello?name=Rana`. This is the Query that extrac the query parameter from the url .
// Query is tuple (T) T is generic that impl deserialize
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    //{:<20} this is string fromater fomat string shift 20 character left  "HANDLER" will place in place holder
    println!("->> {:<20} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    let age = params.age.unwrap().abs();
    use pluralizer::pluralize;
    
    Html(format!("Hello <strong>{}!</strong>. Your age is:<strong> {} </strong> ", first_character_uppercase(name), pluralize("year", age as isize, true)))
}

// e.g `hello2/Rana` extract the variable from the url path
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<20} - handler_hello - {name:?}", "HANDLER");
    Html(format!(
        "Hello <strong>{}</strong>",
        first_character_uppercase(name.as_str())
    ))
}

// function that convet first character upercase of handler_hello2 function name parameter
// USE IN `handler_hello2` FUNCTION
fn first_character_uppercase(s: &str) -> String {
    let mut first = s.chars(); // convert str into char iterator.

    let result = match first.next() {
        None => String::new(), // "" string will be return for match None
        Some(ch) => ch.to_uppercase().collect::<String>() + first.as_str(), //when a match of first character of s.char() convert the upper case and consume to string then remainig characte of first will add as str.
    };
    result

    /*
       if first  =  char(['r', 'a', 'n' , 'a'])
       assert_eq!(first.as_str(), "rana")
       first.next()
       assert_eq(first.as_str(), "ana")
       first.next()
       assert_eq(first.as_str(), "na")
       first.next()
       first.next()
       assert_eq(first.as_str(), "")

    */
}
