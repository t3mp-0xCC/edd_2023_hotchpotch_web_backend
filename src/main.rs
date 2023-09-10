use actix_cors::Cors;
use actix_web::{
    App,
    HttpServer,
    http::header,
};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;

mod auth;
mod cruds;
mod db;
mod models;
mod router;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    dotenv().ok();
    HttpServer::new(||{
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(router::index)
            .service(router::create_user)
            .service(router::get_user)
            .service(router::create_event)
            .service(router::get_event)
            .service(router::delete_event)
            .service(router::create_solo)
            .service(router::get_solo)
            .service(router::create_team)
            .service(router::get_team)
            .service(router::get_team_by_event)
            .service(router::create_join)
            .service(router::create_request)
            .service(router::get_request)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
