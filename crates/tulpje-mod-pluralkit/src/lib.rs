use twilight_model::{
    application::{command::CommandType, interaction::InteractionContextType},
    guild::Permissions,
};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    Module, ModuleBuilder, handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder},
    service_func,
};

use tulpje_lib::context::Services;

mod commands;
mod db;
mod front_update_service;
mod fronters;
mod notify;
mod roles;
mod tasks;
mod util;

pub fn build() -> Module<Services> {
    // define metrics
    metrics::describe_counter!("pk:tracked-systems", "Systems Tracked");
    metrics::describe_counter!("pk:total-systems", "Total Systems Stored");
    metrics::describe_counter!("pk:notifications", "Front Notification Stats");
    metrics::describe_counter!("pk:front-category", "Front Category Stats");

    ModuleBuilder::<Services>::new("pluralkit")
        // commands
        .command(
            CommandBuilder::new("pk", "PluralKit related commands", CommandType::ChatInput)
                .default_member_permissions(Permissions::MANAGE_GUILD)
                .contexts([InteractionContextType::Guild])
                .subcommand(
                    SubCommandBuilder::new("setup", "set-up the PluralKit module")
                        .option(
                            StringBuilder::new("system_id", "PluralKit system ID").required(true),
                        )
                        .handler(handler_func!(commands::setup_pk)),
                )
                .group(roles::commands())
                .group(fronters::commands())
                .group(notify::commands()),
        )
        // tasks
        .task(
            "pk:cleanup-systems",
            "@daily", // once a day at midnight
            handler_func!(tasks::cleanup_systems),
        )
        // services
        .service(
            "pk:update-fronters",
            service_func!(front_update_service::start),
        )
        .build()
}
