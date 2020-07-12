use actix_web::{get, web, App, HttpResponse, HttpServer, Result};

mod pokemon_response;

#[get("/pokemon/{name}")]
async fn get_pokemon_description(pokemon_name: web::Path<String>) -> Result<HttpResponse> {
    let pokemon_name = &pokemon_name.to_string();
    let pokemon_descr = poke_speare::get_description(pokemon_name).await;
    pokemon_response::get(pokemon_name, pokemon_descr).await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(get_pokemon_description))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
