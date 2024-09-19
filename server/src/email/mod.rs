use crate::file::SmtpConfig;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::client::{AsyncSmtpConnection, TlsParameters};
use lettre::transport::smtp::extension::ClientId;
use lettre::Address;
use lettre::Message;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;
use lettre::message::header::ContentType;
use thiserror::Error;
use tracing::{debug, error, trace};

pub mod template;
pub mod ipv4;

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

pub async fn send_email(
    smtp_config: &SmtpConfig,
    local_addr4: Ipv4Addr,
    to: &str,
    body: String,
    reply_to_name: String,
    reply_to_email: &str,
    commission: &str,
    attachments: Vec<Attachment>,
) -> Result<(), SendError> {
    let mb_to = Mailbox::from_str(to)?;

    let mb_from = Mailbox::new(
        Some(smtp_config.from_name.clone()),
        Address::from_str(&smtp_config.from_email)?,
    );

    let mb_reply_to = Mailbox::new(
        Some(reply_to_name),
        Address::from_str(&reply_to_email)?,
    );

    let msg = Message::builder()
        .reply_to(mb_reply_to)
        .from(mb_from)
        .to(mb_to)
        .subject(format!("[DigiDecs] Nieuwe declaratie: {commission}"));

    let mut mp = MultiPart::mixed().build();
    mp = mp.singlepart(SinglePart::html(body));

    for att in attachments {
        mp = mp.singlepart(lettre::message::Attachment::new(att.name)
            .body(att.content, ContentType::parse(&att.mime)?)
        );
    }

    let msg = msg.multipart(mp)?;

    let client_id =
        ClientId::Domain(get_ehlo_domain(&smtp_config.from_email).ok_or(SendError::EmailParse)?);

    trace!("Opening SMTP connection");
    let mut conn = AsyncSmtpConnection::connect_tokio1(
        (smtp_config.smtp_relay.as_str(), 587),
        Some(Duration::from_secs(3)),
        &client_id,
        // We cannot do STARTTLS (which uses port 465, which is blocked by Hetzner), so use port 587
        // Port 587 starts out with regular SMTP commands, after the EHLO we upgrade to STARTTLS
        None,
        Some(IpAddr::V4(local_addr4)),
    )
        .await?;

    if conn.can_starttls() {
        conn.starttls(
            TlsParameters::new_rustls(smtp_config.smtp_relay.as_str().into())?,
            &client_id,
        )
            .await?;
    }

    trace!("Checking SMTP connection");
    if conn.test_connected().await {
        debug!("SMTP connection OK");
    } else {
        error!("Could not connect to server (SMTP)");
        return Err(SendError::Connect);
    }

    trace!("Sending email");
    conn.send(msg.envelope(), &msg.formatted()).await?;

    Ok(())
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
