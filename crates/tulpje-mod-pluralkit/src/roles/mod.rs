use tulpje_framework::{
    handler_func,
    module::command_builder::{SubCommandBuilder, SubCommandGroupBuilder},
};
use twilight_util::builder::command::StringBuilder;

use tulpje_lib::context::Services;

mod commands;
mod constants;
mod prompts;
mod settings;
mod shared;
mod update;
mod update_stats;

use self::commands::setup;

pub(crate) fn commands() -> SubCommandGroupBuilder<Services> {
    SubCommandGroupBuilder::new("roles", "manage member roles")
        .subcommand(
            SubCommandBuilder::new("setup", "set up member roles")
                .handler(handler_func!(setup::handle)),
        )
        .subcommand(
            SubCommandBuilder::new(
                "update",
                "updates member roles to match the configured system",
            )
            .option(StringBuilder::new("token", "(optional) PluralKit token"))
            .handler(handler_func!(update::handle)),
        )
}
