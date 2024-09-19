use crate::args::AppArgs;
use crate::file::AppConfig;
use actix_web::web;
use std::net::Ipv4Addr;

pub type WConfig = web::Data<AppConfig>;

pub type WArgs = web::Data<AppArgs>;

pub type WRuntime = web::Data<RuntimeData>;

#[derive(Clone)]
pub struct RuntimeData {
    pub local_v4_addr: Ipv4Addr,
}
