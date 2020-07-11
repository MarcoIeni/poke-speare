pub use ps_error::PSResult;

mod pokemon;
mod ps_error;
mod shakespeare;

#[cfg(test)]
mod test_utils;

pub async fn get_description(pokemon_name: &str) -> PSResult<String> {
    let original_description = pokemon::get_description(pokemon_name).await?;
    println!("original pokemon description: {}", original_description);
    shakespeare::translate(&original_description).await
}
