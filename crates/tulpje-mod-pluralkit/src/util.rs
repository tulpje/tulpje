use std::str::FromStr;

use pluralkit_rs::models::{Member, SystemRef};
use tulpje_framework::{Error, color};

use tulpje_lib::{context::CommandContext, responses};

pub(crate) fn get_member_name(member: &Member) -> String {
    member
        .display_name
        .clone()
        .unwrap_or_else(|| member.name.clone())
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
}
