use twilight_model::channel::message::Component;
use twilight_util::builder::message::TextDisplayBuilder;

use super::settings::Settings;

pub(super) fn settings_display(settings: &Settings) -> Vec<Component> {
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
