// use crate::{Error, Result};
// use axum::{routing::post, Json, Router};
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};

// pub fn routes() -> Router {
//     Router::new().route("/api/login", post(api_login))
// }

// async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
//     // TODO: Implement real db/auth logic.
//     if payload.username != "demo1" || payload.pwd != "welcome" {
//         return Err(Error::LoginFail);
//     }

//     // TODO : Set cookies

//     // Create the success body.
//     let body = Json(json!({
//         "result" : {
//             "success":true
//         }
//     }));

//     Ok(body)
//     // todo!()
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct LoginPayload {
//     username: String,
//     pwd: String,
// }
