use crate::email::EmailLanguage;
use handlebars::{Handlebars, RenderError};
use serde::Serialize;

static TREASURER_TEMPLATE: &str = include_str!("templates/treasurer.hbs");
static SUBMITTER_NL_TEMPLATE: &str = include_str!("templates/submitter_nl.hbs");
static SUBMITTER_EN_TEMPLATE: &str = include_str!("templates/submitter_en.hbs");
static HEADER_PARTIAL: &str = include_str!("templates/header.partial.hbs");

#[derive(Debug, Serialize)]
pub struct TreasurerData {
    pub name: String,
    pub iban: String,
    pub email: String,
    pub value: String,
    pub what: String,
    pub commission: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SubmitterData {
    pub first_name: String,
}

pub fn render_treasurer(data: &TreasurerData) -> Result<String, RenderError> {
    render_template(TREASURER_TEMPLATE, &data)
}

pub fn render_submitter(
    data: &SubmitterData,
    locale: &EmailLanguage,
) -> Result<String, RenderError> {
    let t = match locale {
        EmailLanguage::Nl => SUBMITTER_NL_TEMPLATE,
        EmailLanguage::En => SUBMITTER_EN_TEMPLATE,
    };

    render_template(t, &data)
}

fn render_template<T: Serialize>(template: &str, data: &T) -> Result<String, RenderError> {
    let mut engine = Handlebars::new();
    engine.set_strict_mode(true);
    engine.register_partial("header", HEADER_PARTIAL)?;

    #[cfg(debug_assertions)]
    engine.set_dev_mode(true);

    engine.render_template(template, data)
}
