use twilight_model::{
    application::{command::CommandType, interaction::InteractionContextType},
    guild::Permissions,
};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    Module, ModuleBuilder, handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder},
};

use tulpje_lib::{context::Services, wizard_component, wizard_modal};

mod commands;
mod db;
mod fronters;
mod notify;
mod roles;
mod tasks;
mod util;
mod wizard;

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
        .command(
            CommandBuilder::new(
                "wizard-test",
                "Testing wizard stufff",
                CommandType::ChatInput,
            )
            .default_member_permissions(Permissions::MANAGE_GUILD)
            .contexts([InteractionContextType::Guild])
            .handler(handler_func!(wizard::handle)),
        )
        // role setup
        .component(
            roles::setup::custom_ids::COMPONENT_NEAR_ROLE_LIMIT_ACCEPT,
            wizard_component!(roles::setup::wizard::AcceptNearRoleLimit),
        )
        .component(
            roles::setup::custom_ids::COMPONENT_NEAR_ROLE_LIMIT_DENY,
            wizard_component!(roles::setup::wizard::DenyNearRoleLimit),
        )
        .component(
            roles::setup::custom_ids::COMPONENT_LEGACY_ROLE_CLEANUP_ACCEPT,
            wizard_component!(roles::setup::wizard::AcceptLegacyRolesCleanup),
        )
        .component(
            roles::setup::custom_ids::COMPONENT_LEGACY_ROLE_CLEANUP_DENY,
            wizard_component!(roles::setup::wizard::DenyLegacyRolesCleanup),
        )
        .modal(
            roles::setup::custom_ids::MODAL_ROLE_SUFFIX_SUBMIT,
            wizard_modal!(roles::setup::wizard::AnswerRoleSuffix),
        )
        // example wizard
        .component(
            wizard::COMPONENT_CLEANUP_WIZARD_CONFIRM,
            wizard_component!(wizard::ConfirmStep),
        )
        .component(
            wizard::COMPONENT_CLEANUP_WIZARD_DENY,
            wizard_component!(wizard::DenyStep),
        )
        // tasks
        .task(
            "pk:update-fronters",
            "*/5 * * * * *", // every 5 seconds
            handler_func!(fronters::tasks::update_fronters),
        )
        .task(
            "pk:cleanup-systems",
            "@daily", // once a day at midnight
            handler_func!(tasks::cleanup_systems),
        )
        .build()
}
