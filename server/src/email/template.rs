use handlebars::{Handlebars, RenderError};
use serde::Serialize;

static TEMPLATE: &str = include_str!("templates/email_template.hbs");

#[derive(Debug, Serialize)]
pub struct TemplateData {
    pub name: String,
    pub iban: String,
    pub email: String,
    pub value: String,
    pub what: String,
    pub commission: String,
    pub notes: Option<String>,
}

pub fn render_template(
    data: &TemplateData,
) -> Result<String, RenderError> {
    let mut engine = Handlebars::new();
    engine.set_strict_mode(true);

    #[cfg(debug_assertions)]
    engine.set_dev_mode(true);

    engine.render_template(TEMPLATE, data)
}