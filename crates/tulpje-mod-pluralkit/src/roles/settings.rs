use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub(super) struct Settings {
    pub(super) suffix: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            suffix: Some(" (Member)".into()),
        }
    }
}
