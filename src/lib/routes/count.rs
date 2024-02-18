// src/lib/routes/count.rs

// dependencies
use actix_web::{post, Responder, Result, web};
use serde::Serialize;

// struct type for the JSON response body
#[derive(Clone, Debug, Serialize)]
struct Count {
  result: String,
}

// count endpoint handler; will eventually return the word count, right now just echos back plaintext input in JSON format
#[post("/count")]
async fn count(body: String) -> Result<impl Responder> {
    let count = Count {
      result: body.to_string()
    };

    Ok(web::Json(count))
}
