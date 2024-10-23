use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod attachment;
mod complete;
mod start;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/digidecs")
                .route("/start", web::post().to(start::start))
                .route("/attachment", web::post().to(attachment::attachment))
                .route("/complete", web::post().to(complete::complete)),
        );
    }
}
