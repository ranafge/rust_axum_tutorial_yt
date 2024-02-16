use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use crate::ctx::Ctx;

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth -{ctx:?}", "MIDDLEWARE");
    ctx?;
    Ok(next.run(req).await)
} 