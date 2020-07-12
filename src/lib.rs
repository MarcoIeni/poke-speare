pub use ps_error::{PSError, PSResult};

mod pokemon;
mod ps_error;
mod shakespeare;

// this is not useful for the library itself, but I have to declare it here
// because otherwise I cannot implement ResponseError for PSError due to the
// rust orphan rules
pub mod server_error;

#[cfg(test)]
mod test_utils;

pub async fn get_description(pokemon_name: &str) -> PSResult<String> {
    let original_description = pokemon::get_description(pokemon_name).await?;
    println!("original pokemon description: {}", original_description);
    shakespeare::translate(&original_description).await
}
