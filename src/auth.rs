use std::sync::Arc;

use hyper::header::{AUTHORIZATION, WWW_AUTHENTICATE};
use hyper::http::HeaderValue;
use hyper::{Body, Request, Response};
use tokio::sync::RwLock;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

struct UserDatabase {
    users: Vec<(String, String)>,
 }

impl UserDatabase {}
hi
async fn handle_request() -> () {
    // let auth_header = req.headers().get(AUTHORIZATION);
    // match auth_header {
    //     Some(header_value) => {}
    //     None => Ok(Response::builder()),
    // };
}

// let auth_header = req.headers().get(AUTHORIZATION);
//
// match auth_header{};
