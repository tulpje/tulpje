use pkrs_fork::{client::PluralKitError, model::PkId};
use tulpje_framework::Error;
use twilight_model::channel::message::component::ButtonStyle;

use tulpje_lib::{
    confirmation_dialog::{ConfirmationDialogBuilder, MessageStyle},
    context::CommandContext,
    responses,
};

use super::{
    db::{self, ModPkSystem},
    util::handle_system_ref,
};

// TODO: command to see current settings
pub async fn setup_pk(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;
    let user_id = ctx.event.author_id().ok_or("no author?")?;

    let Some(system_ref) = handle_system_ref(&ctx, &ctx.get_arg_string("system_id")?).await? else {
        return Ok(());
    };

    let system: ModPkSystem = match ctx
        .services
        .pk
        .get_system(&PkId(system_ref.clone().into()))
        .await
    {
        Ok(system) => system.into(),
        Err(PluralKitError::Pk(_, error))
            // 20001 = System not found
            if error.code == 20001 =>
        {
            responses::error(
                &ctx,
                &format!("### Error\nCouldn't find system `{system_ref}`"),
            )
            .await?;

            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    if let Some(existing_system) = db::get_system_for_guild(&ctx.services.db, guild.id).await?
        && !handle_overwrite_existing_system(&ctx, &existing_system).await?
    {
        // setup was canceled, return
        return Ok(());
    }

    tulpje_lib::db::touch_guild(&ctx.services.db, guild.id).await?;
    db::update_system(&ctx.services.db, &system).await?;
    db::save_guild_settings(&ctx.services.db, guild.id, user_id, system.uuid).await?;

    // Inform user of success
    responses::success(
        &ctx,
        &format!(
            "### Success\nPluralKit module setup for {}",
            system.name.map_or_else(
                || format!("`{}`", system.id),
                |system_name| format!("{} (`{}`)", system_name, system.id)
            )
        ),
    )
    .await?;

    Ok(())
}

async fn handle_overwrite_existing_system(
    ctx: &CommandContext,
    system: &ModPkSystem,
) -> Result<bool, Error> {
    ConfirmationDialogBuilder::new()
        .prompt_text(
            MessageStyle::Warning,
            &format!(
                "### Warning\nThis server is already configured for `{}`\nOverwrite?",
                system.name.as_ref().unwrap_or(&system.id)
            ),
        )
        .cancel_text(
            MessageStyle::Info,
            &format!(
                "### Canceled\nSetup canceled, keeping `{}` as configured system for this server",
                system.name.as_ref().unwrap_or(&system.id)
            ),
        )
        .confirm_button(ButtonStyle::Danger, |builder| {
            builder.label("Yes, overwrite")
        })
        .cancel_button(ButtonStyle::Secondary, |builder| {
            builder.label("No, cancel")
        })
        .build()
        .execute(ctx)
        .await
}
