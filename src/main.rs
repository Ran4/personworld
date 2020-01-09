use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{http, web, App, HttpServer};

mod errors;
mod handlers;
mod models;
mod repo;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::middleware::Logger;

    env_logger::init();

    HttpServer::new(|| {
        let person_service = web::scope("/persons")
            .route("/{id}", web::get().to(handlers::get_person))
            .route("", web::post().to(handlers::post_person));

        App::new()
            .wrap(Logger::default())
            .wrap(
                ErrorHandlers::new().handler(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    handlers::render_500,
                ), // .handler(http::StatusCode::BAD_REQUEST, handlers::render_400),
            )
            .service(person_service)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
