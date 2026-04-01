use pkrs_fork::{
    client::{PkClient, PluralKitError},
    model::{Member, PkId},
};
use tulpje_framework::Error;
use tulpje_lib::{context::CommandContext, responses};
use twilight_model::channel::message::Component;
use twilight_util::builder::message::TextDisplayBuilder;

use crate::util::SystemRef;

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

pub(super) async fn handle_get_system_members(
    ctx: &CommandContext,
    client: &PkClient,
    system_ref: &SystemRef,
) -> Result<Option<Vec<Member>>, Error> {
    match client
        .get_system_members(&PkId(system_ref.clone().into()))
        .await
    {
        Ok(members) => Ok(Some(members)),
        // private member list
        Err(PluralKitError::Pk(_, message))
            // 30001 = unauthorized to view member list
            if message.code == 30001 =>
        {
            // TODO: Try to fetch system name?
            responses::error(
                ctx,
                &format!("### Error\nMember list for `{system_ref}` is private"),
            )
            .await?;
            Ok(None)
        }
        // missing system
        Err(PluralKitError::Pk(_, message))
            // 20001 = missing system
            if message.code == 20001 =>
        {
            responses::error(
                    ctx,
                    &format!("### Error\nPluralKit can't find this system, does `{system_ref}` exist?"),
                )
                .await?;
            Ok(None)
        }
        // miscellaneous errors
        Err(err) => Err(err.into()),
    }
}
