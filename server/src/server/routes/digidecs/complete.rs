use crate::email::template::{render_submitter, render_treasurer, SubmitterData, TreasurerData};
use crate::email::{send_submitter_email, send_treasurer_email, EmailLanguage, TreasurerEmailData};
use crate::server::types::{Empty, Error, Locale, WArgs, WConfig, WResult, WRuntime};
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
    let mut lock = runtime.pending_digidecs.lock().await;
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

    let treasurer = render_treasurer(&TreasurerData {
        name: digidecs.data.name.clone(),
        iban: digidecs.data.iban.clone(),
        email: digidecs.data.email.clone(),
        value: format!("{:.2}", digidecs.data.value),
        what: digidecs.data.what.clone(),
        commission: digidecs.data.commission.clone(),
        notes: digidecs.data.notes.clone(),
    })?;

    let submitter = render_submitter(
        &SubmitterData {
            first_name: digidecs
                .data
                .name
                .split(" ")
                .collect::<Vec<_>>()
                .first()
                .map(|s| s.to_string())
                .unwrap_or(digidecs.data.name.clone()),
        },
        &map_locale_to_email_lang(&digidecs.data.locale),
    )?;

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
        info!("Email body to treasurer: \n{treasurer}");
        info!("Email body to submitter: \n{submitter}");
    } else {
        trace!("Sending Digidecs email to treasurer");
        send_treasurer_email(
            &config.smtp,
            runtime.local_v4_addr,
            TreasurerEmailData {
                to: &config.treasurer_email,
                body: &treasurer,
                reply_to_name: &digidecs.data.name,
                reply_to_email: &digidecs.data.email,
                commission: &digidecs.data.what.clone(),
                attachments,
            },
        )
        .await?;

        trace!("Sending DigiDecs email to submitter");
        send_submitter_email(
            &config.smtp,
            runtime.local_v4_addr,
            &digidecs.data.email,
            submitter,
            &digidecs.data.name,
            &map_locale_to_email_lang(&digidecs.data.locale),
        )
        .await?;
    }

    Ok(Empty)
}

fn map_locale_to_email_lang(locale: &Locale) -> EmailLanguage {
    match locale {
        Locale::En => EmailLanguage::En,
        Locale::Nl => EmailLanguage::Nl,
    }
}
