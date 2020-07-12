use actix_web::{HttpResponse, ResponseError, Result};
use poke_speare::PSResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct PokemonResponse {
    name: String,
    description: String,
}

pub async fn get(
    pokemon_name: &str,
    pokemon_description: PSResult<String>,
) -> Result<HttpResponse> {
    match pokemon_description {
        Ok(description) => Ok(HttpResponse::Ok().json(PokemonResponse {
            name: pokemon_name.to_string(),
            description,
        })),
        Err(err) => err.error_response().await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use poke_speare::server_error::ServerError;
    use poke_speare::PSError;

    fn body_bytes(response: &HttpResponse) -> &[u8] {
        let resp_body = match response.body() {
            actix_http::body::ResponseBody::Body(body) => body,
            _ => panic!("bad body"),
        };
        match resp_body {
            actix_http::body::Body::Bytes(bytes) => bytes.as_ref(),
            _ => panic!("body is not bytes"),
        }
    }

    #[actix_rt::test]
    async fn retrieve_pokemon_description_from_server() {
        let pokemon_name = "geodude";
        let pokemon_description = "pokemon description";
        let resp = get(pokemon_name, Ok(pokemon_description.to_string()))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let expected_json = PokemonResponse {
            name: pokemon_name.to_string(),
            description: pokemon_description.to_string(),
        };

        let body = body_bytes(&resp);
        let actual_json: PokemonResponse = serde_json::from_slice(body).unwrap();
        assert_eq!(actual_json, expected_json);
    }

    async fn check_error_status_code(
        ps_error: PSError,
        expected_status_code: StatusCode,
        expected_error: &str,
    ) {
        let pokemon_name = "whatever";
        let pokemon_description = Err(ps_error);
        let resp = get(pokemon_name, pokemon_description).await.unwrap();
        assert_eq!(resp.status(), expected_status_code);

        let expected_json = ServerError {
            error: expected_error.to_string(),
        };

        let body = body_bytes(&resp);
        let actual_json: ServerError = serde_json::from_slice(body).unwrap();
        assert_eq!(actual_json, expected_json);
    }

    #[actix_rt::test]
    async fn inexistent_pokemon_returns_404() {
        check_error_status_code(
            PSError::PokemonNotFound,
            StatusCode::NOT_FOUND,
            "Pokemon not found",
        )
        .await;
    }

    #[actix_rt::test]
    async fn inexistent_pokemon_description_returns_404() {
        check_error_status_code(
            PSError::NoPokemonEnDescription,
            StatusCode::NOT_FOUND,
            "No english description found for this pokemon",
        )
        .await;
    }

    #[actix_rt::test]
    async fn pokeapi_error_returns_500() {
        check_error_status_code(
            PSError::PokeApiError,
            StatusCode::INTERNAL_SERVER_ERROR,
            "PokeAPI error",
        )
        .await;
    }

    #[actix_rt::test]
    async fn shakespeare_error_returns_500() {
        check_error_status_code(
            PSError::ShakespeareError,
            StatusCode::INTERNAL_SERVER_ERROR,
            "Shakespeare translator error",
        )
        .await;
    }

    #[actix_rt::test]
    async fn quota_error_returns_429() {
        check_error_status_code(
            PSError::QuotaError,
            StatusCode::TOO_MANY_REQUESTS,
            "Too much requests. Quota limits reached",
        )
        .await;
    }
}
