use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use log::info;

mod configuration;
mod pokemon_response;

#[get("/pokemon/{name}")]
async fn get_pokemon_description(pokemon_name: web::Path<String>) -> Result<HttpResponse> {
    let config = configuration::get();
    let shakespeare_token = config.shakespeare_token.as_deref();
    let pokemon_name = &pokemon_name.to_string();
    let pokemon_descr = poke_speare::get_description(pokemon_name, shakespeare_token).await;
    pokemon_response::get(pokemon_name, pokemon_descr).await
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = configuration::get();

    let addr = format!("{}:{}", &config.host, &config.port);
    info!("start server on {}", addr);
    HttpServer::new(|| App::new().service(get_pokemon_description))
        .bind(&addr)?
        .run()
        .await
}
