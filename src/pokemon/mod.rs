use anyhow::{anyhow, Result};
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
    fn description(&self) -> Result<String> {
        let descriptions = &self.flavor_text_entries;
        let en_flavor_text: Vec<&FlavorText> = descriptions
            .iter()
            .filter(|d| d.language.name == "en")
            .collect();
        let first_en_flavor_text = en_flavor_text
            .get(0)
            .ok_or_else(|| anyhow!("english description not available"))?;
        let en_description = first_en_flavor_text.flavor_text.clone();
        let cleaned_description = clean_and_make_one_line(&en_description);
        Ok(cleaned_description)
    }
}

pub async fn get_description(pokemon_name: &str) -> Result<String> {
    let request_url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", pokemon_name);
    let response = reqwest::get(&request_url).await?;
    let pokemon: Pokemon = response.json().await?;
    pokemon.description()
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
    use std::{fs::File, io::BufReader};

    #[test]
    fn description_is_correctly_parsed_from_json() {
        let file_path = test_utils::res_dir().join("charizard.json");
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let charizard: Pokemon = serde_json::from_reader(reader).unwrap();
        let expected_descr = "Spits fire that is hot enough to melt boulders. Known to cause forest fires unintentionally.";
        let actual_descr = charizard.description().unwrap();
        assert_eq!(expected_descr, actual_descr);
    }
}
