use std::str::FromStr;
use std::sync::OnceLock;

use actix_web::web;
use iban::Iban;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tracing::instrument;

use crate::server::types::{
    Error, Locale, PendingDigidecs, PendingDigidecsAttachment, PendingDigidecsData, WResult,
    WRuntime,
};

#[derive(Deserialize)]
pub struct StartDigidecsRequest {
    name: String,
    iban: String,
    email: String,
    value: f64,
    what: String,
    commission: String,
    notes: Option<String>,
    attachments: Vec<Attachment>,
    locale: Locale,
}

#[derive(Deserialize)]
pub struct Attachment {
    name: String,
    mime: String,
}

#[derive(Serialize)]
pub struct StartDigidecsResponse {
    tracking_id: String,
    attachments: Vec<AttachmentResponse>,
}

#[derive(Serialize)]
pub struct AttachmentResponse {
    name: String,
    mime: String,
    tracking_id: String,
}

#[instrument(skip_all)]
pub async fn start(
    payload: web::Json<StartDigidecsRequest>,
    runtime: WRuntime,
) -> WResult<web::Json<StartDigidecsResponse>> {
    let payload = payload.into_inner();

    if !validate_email(&payload.email) {
        return Err(Error::InvalidEmail);
    }

    if !validate_iban(&payload.iban) {
        return Err(Error::InvalidIban);
    }

    if payload.attachments.is_empty() {
        return Err(Error::MissingAttachment);
    }

    if payload.value <= 0.0 {
        return Err(Error::ValueNegativeOrZero);
    }

    let tracking_id = gen_tracking_id();
    let attachment_tracking_ids = payload
        .attachments
        .iter()
        .map(|att| AttachmentResponse {
            name: att.name.clone(),
            tracking_id: gen_tracking_id(),
            mime: att.mime.clone(),
        })
        .collect::<Vec<_>>();

    let mut lock = runtime.pending_digidecs.lock().await;

    lock.push(PendingDigidecs {
        expires_at: OffsetDateTime::now_utc() + Duration::hours(1),
        tracking_id: tracking_id.clone(),
        attachment_count: attachment_tracking_ids.len(),
        attachments: attachment_tracking_ids
            .iter()
            .map(|att| PendingDigidecsAttachment {
                name: att.name.clone(),
                tracking_id: att.tracking_id.clone(),
                content: None,
                mime: att.mime.clone(),
            })
            .collect(),
        data: PendingDigidecsData {
            name: payload.name,
            email: payload.email,
            value: payload.value,
            iban: payload.iban,
            notes: payload.notes,
            what: payload.what,
            commission: payload.commission,
            locale: payload.locale,
        },
    });

    Ok(web::Json(StartDigidecsResponse {
        tracking_id,
        attachments: attachment_tracking_ids,
    }))
}

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

fn gen_tracking_id() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

fn validate_email(email: &str) -> bool {
    let regex = EMAIL_REGEX
        .get_or_init(|| Regex::new(r#"[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+"#).unwrap());
    regex.is_match(email)
}

fn validate_iban(iban: &str) -> bool {
    Iban::from_str(iban).is_ok()
}
