use crate::args::AppArgs;
use crate::file::AppConfig;
use actix_web::web;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

pub type WConfig = web::Data<AppConfig>;

pub type WArgs = web::Data<AppArgs>;

pub type WRuntime = web::Data<RuntimeData>;

#[derive(Clone)]
pub struct RuntimeData {
    pub local_v4_addr: Ipv4Addr,
    pub pending_digidecs: Arc<Mutex<Vec<PendingDigidecs>>>,
}

#[derive(Clone)]
pub struct PendingDigidecs {
    pub expires_at: OffsetDateTime,
    pub data: PendingDigidecsData,
    pub tracking_id: String,
    pub attachment_count: usize,
    pub attachments: Vec<PendingDigidecsAttachment>,
}

#[derive(Clone)]
pub struct PendingDigidecsData {
    pub name: String,
    pub iban: String,
    pub email: String,
    pub value: f64,
    pub what: String,
    pub commission: String,
    pub notes: Option<String>,
}

#[derive(Clone)]
pub struct PendingDigidecsAttachment {
    pub name: String,
    pub tracking_id: String,
    pub mime: String,
    pub content: Option<Vec<u8>>,
}
