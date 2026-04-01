use std::collections::HashSet;

use pkrs_fork::model::Member;
use pluralizer::pluralize;
use tulpje_framework::Error;
use tulpje_lib::{
    ConfirmationDialog,
    context::{CommandContext, Services},
    responses,
};
use twilight_model::{
    channel::message::{
        Component,
        component::{Button, ButtonStyle, TextDisplay},
    },
    guild::Guild,
    id::{Id, marker::RoleMarker},
};
use twilight_util::builder::message::ButtonBuilder;

use crate::roles::{
    constants::{DISCORD_ROLE_LIMIT, REMAINING_ROLE_WARNING},
    db::ModPkGuildRole,
};

#[derive(Debug, Clone, PartialEq)]
pub(super) enum RoleLimitResult {
    Ok,
    Over,
    Near,
}

#[derive(Debug, Clone)]
pub(super) struct RoleLimitHandler {
    total_roles: usize,
    member_roles: usize,
    legacy_member_roles: usize,
    members: usize,
}

impl RoleLimitHandler {
    pub(super) fn new(guild: &Guild, member_roles: &[ModPkGuildRole], members: &[Member]) -> Self {
        let member_role_ids: HashSet<Id<RoleMarker>> =
            member_roles.iter().map(|r| *r.role_id).collect();

        Self {
            total_roles: guild.roles.len(),
            member_roles: member_role_ids.len(),
            // INFO: legacy roles are roles ending in " (Alter)" that aren't
            // in the member roles table
            legacy_member_roles: guild
                .roles
                .iter()
                .filter(|r| r.name.ends_with(" (Alter)") && !member_role_ids.contains(&r.id))
                .count(),
            members: members.len(),
        }
    }

    fn non_tulpje_roles(&self) -> usize {
        self.total_roles - self.member_roles - self.legacy_member_roles
    }

    fn roles_with_members(&self) -> usize {
        self.non_tulpje_roles() + self.members
    }

    pub(super) fn check(&self) -> RoleLimitResult {
        let roles_with_members = self.roles_with_members();

        if roles_with_members > DISCORD_ROLE_LIMIT {
            RoleLimitResult::Over
        } else if roles_with_members > DISCORD_ROLE_LIMIT - REMAINING_ROLE_WARNING {
            RoleLimitResult::Near
        } else {
            RoleLimitResult::Ok
        }
    }

    fn near_message(&self) -> String {
        let existing_roles_noun = pluralize("role", self.non_tulpje_roles() as isize, true);
        let members_noun = pluralize("system member", self.members as isize, true);
        let total_roles_noun = pluralize("role", self.roles_with_members() as isize, true);
        let remaining_count = DISCORD_ROLE_LIMIT.saturating_sub(self.roles_with_members());
        let remaining_noun = pluralize("remaining role", remaining_count as isize, false);

        format!(
            "### Warning\n\
            This server currently has {existing_roles_noun}, \
            adding roles for {members_noun} \
            would be a total of {total_roles_noun} \
            which is close to discord's limit of {DISCORD_ROLE_LIMIT} roles,\
            leaving you with {remaining_noun}, is that okay?"
        )
    }

    pub(super) fn get_message(&self) -> String {
        let existing_roles_noun = pluralize("role", self.non_tulpje_roles() as isize, true);
        let members_noun = pluralize("system member", self.members as isize, true);
        let total_roles_noun = pluralize("role", self.roles_with_members() as isize, true);

        match self.check() {
            RoleLimitResult::Ok => String::new(),
            RoleLimitResult::Near => {
                let remaining_count = DISCORD_ROLE_LIMIT.saturating_sub(self.roles_with_members());
                let remaining_noun = pluralize("remaining role", remaining_count as isize, false);

                format!(
                    "### Warning\n\
                    This server currently has {existing_roles_noun}, \
                    adding roles for {members_noun} \
                    would be a total of {total_roles_noun} \
                    which is close to discord's limit of {DISCORD_ROLE_LIMIT} roles,\
                    leaving you with {remaining_noun}, is that okay?"
                )
            }
            RoleLimitResult::Over => {
                let over_limit = self.roles_with_members().saturating_sub(DISCORD_ROLE_LIMIT);

                format!(
                    "### Error\n\
                    This server currently has {existing_roles_noun}, \
                    adding roles for {members_noun} would leave you with \
                    {total_roles_noun} which is {over_limit} more than \
                    discord's limit of {DISCORD_ROLE_LIMIT} roles"
                )
            }
        }
    }

    fn over_message(&self) -> String {
        let existing_roles_noun = pluralize("role", self.non_tulpje_roles() as isize, true);
        let members_noun = pluralize("system member", self.members as isize, true);
        let total_roles_noun = pluralize("role", self.roles_with_members() as isize, true);
        let over_limit = self.roles_with_members().saturating_sub(DISCORD_ROLE_LIMIT);

        format!(
            "### Error\n\
            This server currently has {existing_roles_noun}, \
            adding roles for {members_noun} would leave you with \
            {total_roles_noun} which is {over_limit} more than \
            discord's limit of {DISCORD_ROLE_LIMIT} roles"
        )
    }

    pub(super) async fn handle(
        &self,
        ctx: &CommandContext,
        command: RoleCommand,
    ) -> Result<bool, Error> {
        match self.check() {
            RoleLimitResult::Ok => Ok(true),
            RoleLimitResult::Near => {
                NearRoleLimitPrompt::new(self.clone(), command)
                    .run(ctx)
                    .await
            }
            RoleLimitResult::Over => {
                responses::error(ctx, &self.over_message()).await?;
                Ok(false)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum RoleCommand {
    Setup,
    Update,
}

#[derive(Debug)]
pub(super) struct NearRoleLimitPrompt {
    limit_handler: RoleLimitHandler,
    command: RoleCommand,
}

impl NearRoleLimitPrompt {
    pub(super) fn new(limit_handler: RoleLimitHandler, command: RoleCommand) -> Self {
        Self {
            limit_handler,
            command,
        }
    }
}

#[async_trait::async_trait]
impl ConfirmationDialog<Services> for NearRoleLimitPrompt {
    async fn prompt_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplay {
                id: None,
                content: self.limit_handler.get_message(),
            }
            .into(),
        ])
    }

    async fn deny_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplay {
                id: None,
                content: format!(
                    "### Canceled\nRole {} canceled",
                    match self.command {
                        RoleCommand::Setup => "setup",
                        RoleCommand::Update => "update",
                    }
                ),
            }
            .into(),
        ])
    }

    async fn confirm_button(&self) -> Result<Button, Error> {
        Ok(ButtonBuilder::new(ButtonStyle::Danger)
            .label("Yes, continue")
            .build())
    }
}
