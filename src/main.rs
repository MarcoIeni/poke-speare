use poke_speare;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let pokemon_name = "charizard";
    let pokemon_description = poke_speare::get_description(pokemon_name).await.unwrap();
    println!("charizard description: {}", pokemon_description);

    Ok(())
}
