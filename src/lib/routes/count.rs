// src/lib/routes/count.rs

// dependencies
use crate::parser::parse_sentence;
use actix_web::{post, Error, HttpResponse, Result};
use serde::Serialize;

// struct type for the JSON response body
#[derive(Clone, Debug, Serialize)]
struct WordCount {
    result: usize,
}

// count endpoint handler; takes the plaintext input from the request body, uses a Nom parser to return a vector containing all the words,
// then gets the length of the vector to get the word count
#[post("/count")]
async fn count(body: String) -> Result<HttpResponse, Error> {
    let (_, words) = parse_sentence(&body).map_err(|_| {
        actix_web::error::ErrorInternalServerError("Unable to parse the request body text.")
    })?; // the parse_sentence function gives back the remainder after the Nom parsing, we destructure to get rid of it

    let word_count = WordCount {
        result: words.len(),
    };

    Ok(HttpResponse::Ok().json(word_count))
}
