use crate::ps_error::{PSError, PSResult};
use log::error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pokemon {
    // A list of pokemon descriptions
    flavor_text_entries: Vec<FlavorText>,
}

#[derive(Deserialize, Debug)]
struct FlavorText {
    flavor_text: String,
    language: Language,
}

#[derive(Deserialize, Debug)]
struct Language {
    name: String,
}

impl Pokemon {
    fn description(&self) -> PSResult<String> {
        let descriptions = &self.flavor_text_entries;
        let first_en_flavor_text = descriptions
            .iter()
            .find(|d| d.language.name == "en")
            .ok_or(PSError::NoPokemonEnDescription)?;
        let en_description = &first_en_flavor_text.flavor_text;
        let cleaned_description = clean_and_make_one_line(en_description);
        Ok(cleaned_description)
    }
}

fn pokemon_path(pokemon_name: &str) -> String {
    format!("/api/v2/pokemon-species/{}", pokemon_name)
}

pub async fn get_description(pokemon_name: &str) -> PSResult<String> {
    let pokemon_path = pokemon_path(pokemon_name);
    let request_url = format!("https://pokeapi.co{}", pokemon_path);
    retrieve_description(&request_url).await
}

async fn retrieve_description(request_url: &str) -> PSResult<String> {
    let response = reqwest::get(request_url).await.map_err(|e| {
        error!("while making pokeapi request: {}", e);
        PSError::PokeApiError
    })?;

    let status = response.status();

    match status.as_u16() {
        200 => {
            let pokemon: Pokemon = response.json().await.map_err(|e| {
                error!("while interpreting shakespeare json payload: {}", e);
                PSError::PokeApiError
            })?;
            pokemon.description()
        }
        404 => Err(PSError::PokemonNotFound),
        429 => Err(PSError::QuotaError),
        _ => {
            error!(
                "pokeapi response: unexpected status code. request_url: {}, response: {:#?}",
                request_url, response
            );
            Err(PSError::PokeApiError)
        }
    }
}

fn clean_and_make_one_line(descr: &str) -> String {
    let cleaned_lines: Vec<String> = descr
        .lines()
        .map(|l| l.trim().replace("\u{c}", " ")) // trim and remove FORM FEED
        .collect();
    cleaned_lines.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use serde_json::Value;
    use std::fs;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn pokemon_description_is_retrieved() {
        let mock_server = MockServer::start().await;

        let file_path = test_utils::res_dir().join("charizard.json");

        let charizard_body = fs::read_to_string(file_path).unwrap();
        let charizard_body: Value = serde_json::from_str(&charizard_body).unwrap();
        let response = ResponseTemplate::new(200).set_body_json(charizard_body);

        let charizard_path = pokemon_path("charizard");
        let request_url = format!("{}{}", &mock_server.uri(), &charizard_path);
        dbg!(&request_url);

        Mock::given(method("GET"))
            .and(path(&charizard_path))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let actual_descr = retrieve_description(&request_url).await.unwrap();
        let expected_descr = "Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.";
        assert_eq!(expected_descr, actual_descr);
    }

    #[tokio::test]
    async fn report_error_if_inexistent_pokemon() {
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(404).set_body_string("Not found");

        let inexistent_pokemon_path = pokemon_path("garurumon");
        let request_url = format!("{}{}", &mock_server.uri(), &inexistent_pokemon_path);
        dbg!(&request_url);

        Mock::given(method("GET"))
            .and(path(&inexistent_pokemon_path))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let response = retrieve_description(&request_url).await;
        let expected_err = Err(PSError::PokemonNotFound);
        assert_eq!(expected_err, response);
    }
}
