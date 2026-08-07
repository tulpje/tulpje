use twilight_model::channel::message::Component;
use twilight_util::builder::message::{ContainerBuilder, TextDisplayBuilder};

use crate::color::{self, Color};

pub type StdError = Box<dyn std::error::Error + Send + Sync>;

// TODO: Naming and error phrasing
#[derive(Debug, thiserror::Error)]
pub enum UserFacingError {
    /// not a programmatic error but a custom message for the user nonetheless
    #[error("user message: {0}")]
    Input(UserMessage),

    /// error with a custom message for the user
    #[error("{1}")]
    WithSource(UserMessage, #[source] StdError),
}

impl UserFacingError {
    pub fn components(&self) -> Vec<Component> {
        match self {
            Self::Input(msg) | Self::WithSource(msg, _) => msg.components(),
        }
    }
}

#[derive(Debug)]
pub enum UserMessage {
    Simple(Option<Color>, String, String),
    Custom(Vec<Component>),
}

impl UserMessage {
    pub fn simple(title: String, body: String) -> Self {
        Self::Simple(None, title, body)
    }

    pub fn with_color(color: Color, title: String, body: String) -> Self {
        Self::Simple(Some(color), title, body)
    }

    pub fn custom(components: Vec<Component>) -> Self {
        Self::Custom(components)
    }

    // default style options
    pub fn error(title: String, body: String) -> Self {
        Self::with_color(color::roles::RED, title, body)
    }

    pub fn warn(title: String, body: String) -> Self {
        Self::with_color(color::roles::ORANGE, title, body)
    }

    pub fn info(title: String, body: String) -> Self {
        Self::with_color(color::roles::BLUE, title, body)
    }

    pub fn components(&self) -> Vec<Component> {
        match self {
            Self::Simple(color, title, body) => vec![
                ContainerBuilder::new()
                    .accent_color(color.map(|c| *c))
                    .component(TextDisplayBuilder::new(format!("### {title}\n{body}")).build())
                    .build()
                    .into(),
            ],
            Self::Custom(components) => components.clone(),
        }
    }
}

impl std::fmt::Display for UserMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(_, title, _) => write!(f, "{title}"),
            Self::Custom(_) => write!(f, "user facing error"),
        }
    }
}
