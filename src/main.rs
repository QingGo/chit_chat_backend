use actix_web::{middleware, App, HttpServer};
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
mod model;
mod router;
mod utils;
use router::router_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(router_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
