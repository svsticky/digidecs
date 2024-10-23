use actix_web::web;
use serde::Deserialize;
use tracing::instrument;

use crate::server::types::{Empty, Error, WResult, WRuntime};

#[derive(Deserialize)]
pub struct Query {
    tracking_id: String,
    attachment_tracking_id: String,
}

#[instrument(skip_all)]
pub async fn attachment(
    query: web::Query<Query>,
    payload: web::Bytes,
    runtime: WRuntime,
) -> WResult<Empty> {
    let mut lock = runtime.pending_digidecs.lock().unwrap();
    let digidecs = lock
        .iter_mut()
        .find(|digidecs| digidecs.tracking_id.eq(&query.tracking_id))
        .ok_or(Error::UnknownTrackingId)?;

    let attachment = digidecs
        .attachments
        .iter_mut()
        .find(|att| att.tracking_id.eq(&query.attachment_tracking_id))
        .ok_or(Error::UnknownAttachmentTrackingId)?;

    attachment.content = Some(payload.to_vec());

    Ok(Empty)
}
