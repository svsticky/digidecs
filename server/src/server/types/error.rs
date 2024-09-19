use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

pub type WResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send email: {0}")]
    Email(#[from] crate::email::SendError),
    #[error("Failed to render email body: {0}")]
    TemplateRender(#[from] handlebars::RenderError),
    #[error("Invalid IBAN")]
    InvalidIban,
    #[error("Invalid Email address")]
    InvalidEmail,
    #[error("At least one attachment is required")]
    MissingAttachment,
    #[error("Value may not be negative or zero")]
    ValueNegativeOrZero,
    #[error("Attachment contains invalid base64")]
    InvalidAttachmentBase64(#[from] base64::DecodeError),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Email(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TemplateRender(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidIban => StatusCode::BAD_REQUEST,
            Self::InvalidEmail => StatusCode::BAD_REQUEST,
            Self::MissingAttachment => StatusCode::BAD_REQUEST,
            Self::ValueNegativeOrZero => StatusCode::BAD_REQUEST,
            Self::InvalidAttachmentBase64(_) => StatusCode::BAD_REQUEST,
        }
    }
}
