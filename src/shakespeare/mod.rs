use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Translation {
    success: Success,
    contents: Contents,
}

#[derive(Deserialize, Debug)]
struct Contents {
    translated: String,
}

#[derive(Deserialize, Debug)]
struct Success {
    total: u32,
}

impl Translation {
    fn translated_text(&self) -> Result<String> {
        match self.success.total {
            1 => Ok(self.contents.translated.clone()),
            _ => Err(anyhow!("shakespeare translation error")),
        }
    }
}

pub async fn translate(text: &str) -> Result<String> {
    let params = [("text", text)];
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.funtranslations.com/translate/shakespeare.json")
        .form(&params)
        .send()
        .await?;

    let translation: Translation = response.json().await?;

    translation.translated_text()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use std::{fs::File, io::BufReader};

    #[test]
    fn description_is_correctly_parsed_from_json() {
        let file_path = test_utils::res_dir().join("shakespeare_response.json");
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let translation: Translation = serde_json::from_reader(reader).unwrap();
        let expected_translation = "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did doth englut did maketh him kicketh the bucket.";
        let actual_translation = translation.translated_text().unwrap();
        assert_eq!(expected_translation, actual_translation);
    }
}
