use tulpje_framework::Error;

use super::{db, set_guild_commands_for_guild};
use tulpje_lib::{context::CommandContext, responses};

pub(crate) async fn enable(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    let module = ctx.get_arg_string("module")?;
    if !ctx.services.registry.guild_module_names().contains(&module) {
        responses::error(&ctx, &format!("no module named `{}` exists", module)).await?;
        return Ok(());
    }

    tulpje_lib::db::guild::touch(&ctx.services.db, guild.id).await?;
    db::enable_module(&ctx.services.db, guild.id, &module).await?;
    set_guild_commands_for_guild(
        &db::guild_modules(&ctx.services.db, guild.id).await?,
        guild.id,
        ctx.interaction(),
        &ctx.services.registry,
    )
    .await?;

    responses::success(&ctx, &format!("module `{}` enabled", module)).await?;

    Ok(())
}

pub(crate) async fn disable(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    let module = ctx.get_arg_string("module")?;
    if !ctx.services.registry.guild_module_names().contains(&module) {
        responses::error(&ctx, &format!("no module named `{}` exists", module)).await?;
        return Ok(());
    }

    db::disable_module(&ctx.services.db, guild.id, &module).await?;
    set_guild_commands_for_guild(
        &db::guild_modules(&ctx.services.db, guild.id).await?,
        guild.id,
        ctx.interaction(),
        &ctx.services.registry,
    )
    .await?;

    responses::success(&ctx, &format!("module `{}` disabled", module)).await?;

    Ok(())
}

pub(crate) async fn modules(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    let global_modules = ctx.services.registry.global_module_names();
    let modules = db::guild_modules(&ctx.services.db, guild.id).await?;
    let available: Vec<String> = ctx
        .services
        .registry
        .guild_module_names()
        .into_iter()
        .filter(|m| !modules.contains(m))
        .collect();

    responses::info(
        &ctx,
        &format!(
            "**Enabled: {}**\nAlways Enabled: {}\nAvailable: {}",
            modules.join(", "),
            global_modules.join(", "),
            available.join(", ")
        ),
    )
    .await?;

    Ok(())
}
