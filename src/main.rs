#[tokio::main]
async fn main() {
    env_logger::init();

    let pokemon_name = "charizard";
    let pokemon_description = poke_speare::get_description(pokemon_name).await.unwrap();
    println!("charizard description: {}", pokemon_description);
}
