use tulpje_framework::color;

#[derive(Debug, Clone, Copy)]
pub enum MessageStyle {
    Success,
    Info,
    Warning,
    Danger,
}

impl From<MessageStyle> for color::Color {
    fn from(value: MessageStyle) -> Self {
        match value {
            MessageStyle::Success => color::roles::GREEN,
            MessageStyle::Info => color::roles::BLUE,
            MessageStyle::Warning => color::roles::ORANGE,
            MessageStyle::Danger => color::roles::RED,
        }
    }
}

impl From<MessageStyle> for u32 {
    fn from(value: MessageStyle) -> Self {
        *color::Color::from(value)
    }
}
