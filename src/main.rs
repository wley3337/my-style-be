use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use dotenvy::dotenv as dotenv_core;
use dotenvy_macro::dotenv;

mod api;
use api::user::get_user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv_core().ok();
    env_logger::init();

    let database_uri = dotenv!("DATABASE_URL");
    let server_address = dotenv!("SERVER_ADDRESS");
    let fe_server_address = dotenv!("FRONTEND_SERVER_ADDRESS");
    println!("database_uri: {}", database_uri);
    println!("Started server at address: {}", server_address);
    println!("fe server address: {}", fe_server_address);

    HttpServer::new(|| {
        let logger = Logger::default();
        // add cors
        let cors = Cors::default()
            .allowed_origin(fe_server_address)
            .allow_any_method()
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ORIGIN])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new().service(get_user).wrap(logger).wrap(cors)
    })
    .bind(server_address)?
    .run()
    .await
}
