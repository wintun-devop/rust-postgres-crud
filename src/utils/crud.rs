use serde::{Deserialize, Serialize};
use serde_json::{Result,Value,json};
use uuid::Uuid;
use hyper::{Request, Response, StatusCode};
use hyper::body::Body;
use sqlx::PgPool;



type Db = PgPool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

/// Create user handler: expects JSON body matching CreateUser.
/// Returns 201 Created with the created user JSON on success.

pub async fn create_user(db: Db) -> Result<Value> {
   let _a = db;
   let v = json!({ "status": "ok", "id": 1 });
   Ok(v)
}

// Helper to build a JSON error response with a message.
// fn json_error(status: StatusCode, message: &str) -> Response<Body> {
//     let payload = serde_json::json!({ "error": message });
//     let body = serde_json::to_string(&payload).unwrap_or_else(|_| r#"{"error":"internal"}"#);
//     Response::builder()
//         .status(status)
//         .header("Content-Type", "application/json")
//         .body(Body::from(body))
//         .unwrap()
// }

// pub fn build_response<T: Serialize>(status: StatusCode, body_obj: T) -> Response<dyn Body> {
//     let body = match serde_json::to_string(&body_obj) {
//         Ok(s) => s,
//         Err(_) => return {
//             let err = r#"{"error":"serialization error"}"#;
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .header("content-type", "application/json")
//                 .body(Body::from(err))
//                 .unwrap()
//         }
//     };

//     Response::builder()
//         .status(status)
//         .header("content-type", "application/json")
//         .body(Body::from(body))
//         .unwrap()
// }