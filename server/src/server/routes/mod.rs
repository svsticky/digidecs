use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod digidecs;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/api")
            .route("/digidecs", web::post().to(digidecs::digidecs))
        );
    }
}