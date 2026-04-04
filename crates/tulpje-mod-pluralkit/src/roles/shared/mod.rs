use pkrs_fork::{
    client::{PkClient, PluralKitError},
    model::{Member, PkId},
};
use tulpje_framework::Error;
use tulpje_lib::{context::CommandContext, responses};

use crate::util::SystemRef;

pub(crate) mod settings;

pub(crate) async fn handle_get_system_members(
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
