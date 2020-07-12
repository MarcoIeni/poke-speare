use actix_http::ResponseBuilder;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use crate::PSError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ServerError {
    pub error: Error,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Error {
    pub message: String,
}

impl ResponseError for PSError {
    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        ResponseBuilder::new(self.status_code()).json(ServerError {
            error: Error { message },
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            PSError::PokemonNotFound => StatusCode::NOT_FOUND,
            PSError::NoPokemonEnDescription => StatusCode::NOT_FOUND,
            PSError::QuotaError => StatusCode::TOO_MANY_REQUESTS,
            PSError::ShakespeareError => StatusCode::INTERNAL_SERVER_ERROR,
            PSError::PokeApiError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
