use crate::email::send_email;
use crate::email::template::{render_template, TemplateData};
use crate::server::types::{Empty, Error, WArgs, WConfig, WResult, WRuntime};
use actix_web::web;
use serde::Deserialize;
use time::OffsetDateTime;
use tracing::{info, instrument, trace};

#[derive(Deserialize)]
pub struct Query {
    tracking_id: String,
}

#[instrument(skip_all)]
pub async fn complete(
    query: web::Query<Query>,
    config: WConfig,
    runtime: WRuntime,
    args: WArgs,
) -> WResult<Empty> {
    let mut lock = runtime.pending_digidecs.lock().unwrap();
    let (idx, _) = lock
        .iter()
        .enumerate()
        .find(|(_, digidecs)| digidecs.tracking_id.eq(&query.tracking_id))
        .ok_or(Error::UnknownTrackingId)?;

    let digidecs = lock.remove(idx);

    if digidecs.expires_at <= OffsetDateTime::now_utc() {
        return Err(Error::DigidecsExpired);
    }

    let attachments_cnt = digidecs
        .attachments
        .iter()
        .filter(|att| att.content.is_some())
        .count();

    if digidecs.attachment_count != attachments_cnt {
        return Err(Error::MissingAttachment);
    }

    let email_body = render_template(&TemplateData {
        name: digidecs.data.name.clone(),
        iban: digidecs.data.iban.clone(),
        email: digidecs.data.email.clone(),
        value: format!("{:.2}", digidecs.data.value),
        what: digidecs.data.what.clone(),
        commission: digidecs.data.commission.clone(),
        notes: digidecs.data.notes.clone(),
    })?;

    let attachments = digidecs
        .attachments
        .into_iter()
        .map(|att| crate::email::Attachment {
            name: att.name,
            mime: att.mime,
            content: att.content.unwrap(), // Some state is checked when checking if every attachment has content set earier
        })
        .collect::<Vec<_>>();

    if args.dry_run {
        info!("Dry run is enabled. Not sending email.");
        info!("Email body: \n{email_body}");
    } else {
        trace!("Sending Digidecs email");
        send_email(
            &config.smtp,
            runtime.local_v4_addr,
            &config.treasurer_email,
            email_body,
            digidecs.data.name.clone(),
            &digidecs.data.email.clone(),
            &digidecs.data.what.clone(),
            attachments,
        )
        .await?;
    }

    Ok(Empty)
}
