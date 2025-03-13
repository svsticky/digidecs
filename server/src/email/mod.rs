use crate::file::SmtpConfig;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::client::{AsyncSmtpConnection, TlsParameters};
use lettre::transport::smtp::extension::ClientId;
use lettre::Address;
use lettre::Message;
use rand::Rng;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, trace};

pub mod ipv4;
pub mod template;

pub enum EmailLanguage {
    Nl,
    En,
}

#[derive(Debug, Error)]
pub enum SendError {
    #[error("Failed to parse email address")]
    EmailParse,
    #[error("Failed to parse email address: {0}")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("General error: {0}")]
    General(#[from] lettre::error::Error),
    #[error("Failed to deliver email due to SMTM failure: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),
    #[error("Could not connect to the server")]
    Connect,
    #[error("Invalid MIME type: {0}")]
    ContentType(#[from] lettre::message::header::ContentTypeErr),
}

pub struct Attachment {
    pub content: Vec<u8>,
    pub name: String,
    pub mime: String,
}

pub struct TreasurerEmailData<'a> {
    pub to: &'a str,
    pub body: &'a str,
    pub reply_to_name: &'a str,
    pub reply_to_email: &'a str,
    pub commission: &'a str,
    pub attachments: Vec<Attachment>,
}

pub async fn send_treasurer_email(
    smtp_config: &SmtpConfig,
    local_addr4: Ipv4Addr,
    data: TreasurerEmailData<'_>,
) -> Result<(), SendError> {
    let mb_to = Mailbox::from_str(data.to)?;

    let mb_from = Mailbox::new(
        Some(smtp_config.from_name.clone()),
        Address::from_str(&smtp_config.from_email)?,
    );

    let mb_reply_to = Mailbox::new(
        Some(data.reply_to_name.to_string()),
        Address::from_str(data.reply_to_email)?,
    );

    let msg = Message::builder()
        .reply_to(mb_reply_to)
        .from(mb_from)
        .to(mb_to)
        .subject(format!(
            "[DigiDecs] Nieuwe declaratie: {} ({})",
            data.commission,
            rand::thread_rng()
                .sample_iter(rand::distributions::Alphanumeric)
                .take(6)
                .map(char::from)
                .collect::<String>()
        ));

    let mut mp = MultiPart::mixed().build();
    mp = mp.singlepart(SinglePart::html(data.body.to_string()));

    for att in data.attachments {
        mp = mp.singlepart(
            lettre::message::Attachment::new(att.name)
                .body(att.content, ContentType::parse(&att.mime)?),
        );
    }

    let msg = msg.multipart(mp)?;

    let mut conn = smtp_connect(smtp_config, local_addr4).await?;

    trace!("Sending email");
    conn.send(msg.envelope(), &msg.formatted()).await?;

    Ok(())
}

pub async fn send_submitter_email(
    smtp_config: &SmtpConfig,
    local_addr4: Ipv4Addr,
    to_email: &str,
    body: String,
    name: &str,
    locale: &EmailLanguage,
) -> Result<(), SendError> {
    let mb_to = Mailbox::new(Some(name.to_string()), Address::from_str(to_email)?);

    let mb_from = Mailbox::new(
        Some(smtp_config.from_name.clone()),
        Address::from_str(&smtp_config.from_email)?,
    );

    let msg = Message::builder()
        .to(mb_to)
        .from(mb_from)
        .subject(submitter_subject(locale))
        .singlepart(SinglePart::html(body))?;

    let mut conn = smtp_connect(smtp_config, local_addr4).await?;
    trace!("Sending email");
    conn.send(msg.envelope(), &msg.formatted()).await?;

    Ok(())
}

fn submitter_subject(locale: &EmailLanguage) -> String {
    match locale {
        EmailLanguage::Nl => "Je DigiDecs is ontvangen!".into(),
        EmailLanguage::En => "Your DigiDecs has been received!".into(),
    }
}

async fn smtp_connect(
    config: &SmtpConfig,
    bind_addr: Ipv4Addr,
) -> Result<AsyncSmtpConnection, SendError> {
    let client_id =
        ClientId::Domain(get_ehlo_domain(&config.from_email).ok_or(SendError::EmailParse)?);

    trace!("Opening SMTP connection");
    let mut conn = AsyncSmtpConnection::connect_tokio1(
        (config.smtp_relay.as_str(), 587),
        Some(Duration::from_secs(3)),
        &client_id,
        // We cannot do STARTTLS (which uses port 465, which is blocked by Hetzner), so use port 587
        // Port 587 starts out with regular SMTP commands, after the EHLO we upgrade to STARTTLS
        None,
        Some(IpAddr::V4(bind_addr)),
    )
    .await?;

    if conn.can_starttls() {
        conn.starttls(
            TlsParameters::new_rustls(config.smtp_relay.as_str().into())?,
            &client_id,
        )
        .await?;
    }

    trace!("Checking SMTP connection");
    if conn.test_connected().await {
        debug!("SMTP connection OK");
        Ok(conn)
    } else {
        error!("Could not connect to server (SMTP)");
        Err(SendError::Connect)
    }
}

fn get_ehlo_domain(email: &str) -> Option<String> {
    email.split('@').nth(1).map(|domain| domain.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ehlo_domain() {
        assert_eq!(
            get_ehlo_domain("foo@example.com"),
            Some("example.com".to_owned())
        );
        assert_eq!(get_ehlo_domain("example.org"), None);
        assert_eq!(get_ehlo_domain("example@"), Some(String::new()))
    }
}
