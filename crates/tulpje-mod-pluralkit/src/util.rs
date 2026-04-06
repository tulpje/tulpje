use std::{fmt::Display, str::FromStr};

use pkrs_fork::model::Member;
use tulpje_framework::{Error, color};
use twilight_model::id::{Id, marker::UserMarker};
use unicode_segmentation::UnicodeSegmentation as _;
use uuid::Uuid;

use tulpje_lib::{context::CommandContext, responses};

pub(crate) fn get_member_name(member: &Member) -> String {
    member
        .display_name
        .clone()
        .unwrap_or_else(|| member.name.clone())
}

pub(crate) fn normalize_short_id(short_id: &str) -> String {
    short_id.trim().replace("-", "").to_ascii_lowercase()
}

#[derive(Debug, Clone)]
pub(crate) enum SystemRef {
    DiscordId(Id<UserMarker>),
    Uuid(Uuid),
    Id(String),
}

impl FromStr for SystemRef {
    type Err = tulpje_framework::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // PluralKit short IDs
        if s.len() >= 5 && s.len() <= 7 {
            let normalized = normalize_short_id(s);
            if normalized.len() >= 5 && normalized.len() <= 6 {
                return Ok(Self::Id(normalized));
            }
        }

        // Discord Snowflakes
        if let Ok(discord_id) = Id::<UserMarker>::from_str(s) {
            return Ok(Self::DiscordId(discord_id));
        }

        // UUIDs
        if let Ok(uuid) = Uuid::from_str(s) {
            return Ok(Self::Uuid(uuid));
        }

        Err(format!("Couldn't parse '{s}' into SystemRef").into())
    }
}

impl From<SystemRef> for String {
    fn from(value: SystemRef) -> Self {
        match value {
            SystemRef::Id(id) => id,
            SystemRef::DiscordId(user_id) => user_id.to_string(),
            SystemRef::Uuid(uuid) => uuid.to_string(),
        }
    }
}

impl From<Uuid> for SystemRef {
    fn from(value: Uuid) -> Self {
        Self::Uuid(value)
    }
}

impl Display for SystemRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => id.fmt(f),
            Self::DiscordId(user_id) => user_id.fmt(f),
            Self::Uuid(uuid) => uuid.fmt(f),
        }
    }
}

/// try to parse a system ref, and let the end user know if it fails
/// returns None if failed to parse
pub(crate) async fn handle_system_ref(
    ctx: &CommandContext,
    system_ref: &str,
) -> Result<Option<SystemRef>, Error> {
    match system_ref.parse() {
        Ok(system_ref) => Ok(Some(system_ref)),
        Err(_) => {
            responses::error(
                ctx,
                &format!(
                    "Invalid system reference `{system_ref}`, are you sure you entered it correctly?",
                ),
            )
            .await?;
            Ok(None)
        }
    }
}

pub(crate) fn pk_color_to_discord(hex: Option<String>) -> u32 {
    hex.map_or(color::roles::DEFAULT, |hex| {
        color::Color::from_str(&hex).unwrap_or(color::roles::DEFAULT)
    })
    .0
}

// discord text length limits seem to be based on UTF-16 code units
// this means that we have to ellipsize based on that.
// however we also want to not break up any grapheme clusters so we have to handle that too
pub(crate) fn discord_ellipsize<'a>(
    text: &str,
    max_length: usize, // length in `char`s
    ellipsis_text: impl Into<Option<&'a str>>,
) -> String {
    // calculate length in utf16 code units
    let text_len = text.encode_utf16().count();

    // return full text if it's below limit
    if text_len <= max_length {
        return String::from(text);
    }

    // default option for ellipsis text
    let ellipsis_text = ellipsis_text.into().unwrap_or("...");

    // calculate ellipsis lenggth in utf16 code units
    let ellipsis_len = ellipsis_text.encode_utf16().count();

    // fill up a vec with graphemes, stop before it goes past `max_length`
    let mut total_len = 0;
    let mut graphemes = Vec::new();
    for grapheme in text.graphemes(true) {
        let grapheme_len = grapheme.encode_utf16().count();
        if total_len + grapheme_len > max_length - ellipsis_len {
            break;
        }

        graphemes.push(grapheme);
        total_len += grapheme_len;
    }

    let shortened_text = graphemes.join("");
    format!("{shortened_text}{ellipsis_text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pk_color_to_discord() {
        assert_eq!(
            pk_color_to_discord(Some("unparseable".to_string())),
            color::roles::DEFAULT.0
        );
        assert_eq!(pk_color_to_discord(None), color::roles::DEFAULT.0);
    }

    #[test]
    fn test_graphene_ellipsize() {
        // doesn't shorten strings shorter than max length
        assert_eq!(
            discord_ellipsize("lorem ipsum, uwu", 100, None),
            "lorem ipsum, uwu"
        );

        // shortens strings longer than max length
        assert_eq!(discord_ellipsize("0123456789", 5, None), "01...");

        // handles custom ellipsis text
        assert_eq!(discord_ellipsize("0123456789", 5, "…"), "0123…");

        // handles unicode shenanigans (each trans flag is 5 characters)
        assert_eq!(discord_ellipsize("🏳️‍⚧️🏳️‍⚧️", 9, None), "🏳️‍⚧️...");

        assert_eq!(
            discord_ellipsize("🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️", 100, ""),
            "🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️🏳️‍⚧️"
        );
    }
}
