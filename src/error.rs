use axum::{http::StatusCode, response::IntoResponse};

// associated type Result T is generic . core::result::Result provide the access of Result module to use Result enum
pub type Result<T>  = core::result::Result<T, Error>;


// create error enum 
#[derive(Debug)]
pub enum Error {
    LoginFail
}

// implement the IntoResponse for Error return an axum response
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("{:<12} - {self:?} ", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}