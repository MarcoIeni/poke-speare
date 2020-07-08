use anyhow::Result;

mod pokemon;
mod shakespeare;

#[cfg(test)]
mod test_utils;

pub async fn get_description(pokemon_name: &str) -> Result<String> {
    let original_description = pokemon::get_description(pokemon_name).await?;
    println!("original pokemon description: {}", original_description);
    shakespeare::translate(&original_description).await
}
