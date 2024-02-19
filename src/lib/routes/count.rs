// src/lib/routes/count.rs

// dependencies
use crate::parser::parse_sentence;
use actix_web::{post, web, Result, Responder};
use serde::Serialize;

// for future use in in improving the error handling and getting rid of the unwrap below
// type NomError = nom::Err<nom::error::Error<&str>>;

// struct type for the JSON response body
#[derive(Clone, Debug, Serialize)]
struct WordCount {
    result: usize,
}

// count endpoint handler; takes the plaintext input from the request body, uses a Nom parser to return a vector containing all the words,
// then gets the length of the vector to get the word count
#[post("/count")]
async fn count(body: String) -> Result<impl Responder> {
    let (_, words) = parse_sentence(&body).unwrap();    // the parse_sentence function gives back the remainder after the Nom parsing, we destructure to get rid of it

    let word_count = WordCount {
        result: words.len(),
    };

    Ok(web::Json(word_count))
}
