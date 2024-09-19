use crate::email::send_email;
use crate::email::template::{render_template, TemplateData};
use crate::server::types::{Empty, Error, WArgs, WConfig, WResult, WRuntime};
use actix_web::web;
use iban::Iban;
use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::OnceLock;
use base64::{DecodeError, Engine};
use tracing::{info, instrument};

#[derive(Deserialize)]
pub struct DigidecsRequest {
    pub name: String,
    pub iban: String,
    pub email: String,
    pub value: f64,
    pub what: String,
    pub commission: String,
    pub notes: Option<String>,
    pub attachments: Vec<Attachment>,
}

#[derive(Deserialize)]
pub struct Attachment {
    pub content: String,
    pub name: String,
    pub mime: String,
}

#[instrument(skip_all)]
pub async fn digidecs(
    config: WConfig,
    runtime_config: WRuntime,
    payload: web::Json<DigidecsRequest>,
    args: WArgs,
) -> WResult<Empty> {
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

    let attachments = payload.attachments.into_iter()
        .map(|att| {
            Ok(crate::email::Attachment {
                content: base64::engine::general_purpose::STANDARD.decode(&att.content.as_bytes())?,
                name: att.name,
                mime: att.mime
            })
        })
        .collect::<Result<Vec<_>, DecodeError>>()?;

    let email_body = render_template(&TemplateData {
        name: payload.name.clone(),
        iban: payload.iban,
        email: payload.email.clone(),
        value: format!("{:.2}", payload.value),
        what: payload.what.clone(),
        commission: payload.commission,
        notes: payload.notes,
    })?;

    if args.dry_run {
        info!("Dry run is enabled. Not sending email.");
        info!("Email body: \n{email_body}");
    } else {
        send_email(
            &config.smtp,
            runtime_config.local_v4_addr,
            &config.treasurer_email,
            email_body,
            payload.name,
            &payload.email,
            &payload.what,
            attachments,
        ).await?;
    }



    Ok(Empty)
}

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

fn validate_email(email: &str) -> bool {
    let regex = EMAIL_REGEX.get_or_init(|| Regex::new(r#"[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+"#).unwrap());
    regex.is_match(email)
}

fn validate_iban(iban: &str) -> bool {
    Iban::from_str(iban).is_ok()
}