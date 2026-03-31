use tulpje_framework::Error;
use tulpje_lib::{ConfirmationDialog, context::Services, message_style::MessageStyle};
use twilight_model::channel::message::{
    Component,
    component::{Button, ButtonStyle},
};
use twilight_util::builder::message::{ButtonBuilder, TextDisplayBuilder};

use crate::roles::{constants::DISCORD_ROLE_LIMIT, update_stats::UpdateStats};

pub(crate) fn role_change_message(stats: &UpdateStats, infix: &str) -> String {
    // all this code is just to get messages to look like
    //   1 role created, 2 updated, 1 assigned
    //   2 roles updated, 1 assigned
    //   etc
    let mut parts = Vec::<(u16, &'static str)>::new();
    if stats.create.done > 0 {
        parts.push((stats.create.done, "created"));
    }
    if stats.update.done > 0 {
        parts.push((stats.update.done, "updated"));
    }
    if stats.delete.done > 0 {
        parts.push((stats.delete.done, "deleted"));
    }
    if stats.assign.done > 0 {
        parts.push((stats.assign.done, "assigned"));
    }

    let infix = if !infix.is_empty() {
        &format!("{} ", infix.trim())
    } else {
        infix
    };

    parts
        .into_iter()
        .enumerate()
        .map(|(idx, (count, verb))| {
            if idx == 0 {
                let noun = if count == 1 { "role" } else { "roles" };
                format!("{count} {noun} {infix}{verb}")
            } else {
                format!("{count} {infix}{verb}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) struct NearRoleLimitWarningPrompt {
    system_member_count: usize,
    existing_role_count: usize,
}

impl NearRoleLimitWarningPrompt {
    pub(crate) fn new(system_member_count: usize, existing_role_count: usize) -> Self {
        Self {
            system_member_count,
            existing_role_count,
        }
    }
}

#[async_trait::async_trait]
impl ConfirmationDialog<Services> for NearRoleLimitWarningPrompt {
    async fn prompt_message(&self) -> Result<Vec<Component>, Error> {
        let system_member_count = self.system_member_count;
        let existing_role_count = self.existing_role_count;
        let combined_count = system_member_count + existing_role_count;
        let remaining_count = DISCORD_ROLE_LIMIT.saturating_sub(combined_count);
        let existing_role_noun = if existing_role_count == 1 {
            "role"
        } else {
            "roles"
        };
        let system_member_noun = if system_member_count == 1 {
            "member"
        } else {
            "members"
        };
        let combined_noun = if combined_count == 1 { "role" } else { "roles" };
        let remaining_noun = if remaining_count == 1 {
            "role"
        } else {
            "roles"
        };

        Ok(vec![
            TextDisplayBuilder::new(format!(
                "### Warning\n\
                This server currently has {existing_role_count} {existing_role_noun}, \
                adding roles for {system_member_count} sytem {system_member_noun} \
                would leave you with {combined_count} {combined_noun} \
                which is close to discord's limit of {DISCORD_ROLE_LIMIT} roles,\
                leaving you with {remaining_count} remaining {remaining_noun}, is that okay?"
            ))
            .build()
            .into(),
        ])
    }

    async fn deny_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplayBuilder::new("### Canceled\nRole update canceled")
                .build()
                .into(),
        ])
    }

    async fn confirm_button(&self) -> Result<Button, Error> {
        Ok(ButtonBuilder::new(ButtonStyle::Danger)
            .label("Yes, create roles")
            .build())
    }
}

pub(crate) struct ConfirmUpdatePrompt {
    stats: UpdateStats,
}

impl ConfirmUpdatePrompt {
    pub(crate) fn new(stats: UpdateStats) -> Self {
        Self { stats }
    }
}

#[async_trait::async_trait]
impl ConfirmationDialog<Services> for ConfirmUpdatePrompt {
    const PROMPT_STYLE: MessageStyle = MessageStyle::Info;

    async fn prompt_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplayBuilder::new(format!(
                "### Update Roles?\n{}",
                role_change_message(&self.stats, "will be")
            ))
            .build()
            .into(),
        ])
    }

    async fn deny_message(&self) -> Result<Vec<Component>, Error> {
        Ok(vec![
            TextDisplayBuilder::new("### Canceled\nRole update canceled")
                .build()
                .into(),
        ])
    }
}
