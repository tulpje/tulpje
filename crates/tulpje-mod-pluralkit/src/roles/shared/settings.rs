use serde::{Deserialize, Serialize};
use twilight_model::channel::message::Component;
use twilight_util::builder::message::TextDisplayBuilder;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(crate) struct Settings {
    pub(super) suffix: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            suffix: Some(" (Member)".into()),
        }
    }
}

pub(crate) fn settings_display(settings: &Settings) -> Vec<Component> {
    vec![
        TextDisplayBuilder::new(format!(
            "\
                Member Suffix: {}\n\
                -# Text added at the end of system member names\
            ",
            settings
                .suffix
                .clone()
                .map(|s| format!("`{s}`"))
                .unwrap_or_else(|| "*`empty`*".into())
        ))
        .build()
        .into(),
    ]
}
