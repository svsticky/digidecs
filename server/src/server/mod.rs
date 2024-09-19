use actix_cors::Cors;
use actix_route_config::Routable;
use actix_web::{web, App, HttpServer};
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tracing::info;
use crate::args::AppArgs;
use crate::email::ipv4::get_local_v4;
use crate::file::AppConfig;
use crate::server::types::{RuntimeData, WArgs, WConfig, WRuntime};

mod types;
mod routes;

pub async fn run_server(
    config: AppConfig,
    args: AppArgs,
) -> color_eyre::Result<()> {
    let port = config.server.port;

    let runtime_data = RuntimeData {
        local_v4_addr: get_local_v4().await?,
    };

    info!("Using {} for SMTP connections", runtime_data.local_v4_addr);

    let host = config.server.domain.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(tracing_actix_web::TracingLogger::<NoiselessRootSpanBuilder>::new())
            .app_data(WConfig::new(config.clone()))
            .app_data(WArgs::new(args.clone()))
            .app_data(WRuntime::new(runtime_data.clone()))
            .app_data(web::JsonConfig::default()
                .limit(20*10^6)
                )
            .configure(routes::Router::configure)
    })
        .bind(format!("0.0.0.0:{port}"))?
        .server_hostname(&host)
        .run()
        .await?;

    Ok(())
}