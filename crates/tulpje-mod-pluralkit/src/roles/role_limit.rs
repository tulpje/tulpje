use pluralizer::pluralize;

use crate::roles::constants::{DISCORD_ROLE_LIMIT, REMAINING_ROLE_WARNING};

#[derive(Debug, Clone, PartialEq)]
pub(super) enum RoleLimitResult {
    Ok,
    Over,
    Near,
}

#[derive(Debug, Clone)]
pub(super) struct RoleLimitData {
    pub(super) total_roles: usize,
    pub(super) member_roles: usize,
    pub(super) legacy_member_roles: usize,
    pub(super) members: usize,
}
impl RoleLimitData {
    pub(super) fn new(
        total_roles: usize,
        member_roles: usize,
        legacy_member_roles: usize,
        members: usize,
    ) -> Self {
        Self {
            total_roles,
            member_roles,
            // INFO: legacy roles are roles ending in " (Alter)" that aren't
            // in the member roles table
            legacy_member_roles,
            members,
        }
    }

    pub(super) fn non_tulpje_roles(&self) -> usize {
        self.total_roles - self.member_roles - self.legacy_member_roles
    }

    pub(super) fn roles_with_members(&self) -> usize {
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

    pub(super) fn remaining(&self) -> usize {
        DISCORD_ROLE_LIMIT.saturating_sub(self.roles_with_members())
    }

    pub(super) fn over_limit(&self) -> usize {
        self.roles_with_members().saturating_sub(DISCORD_ROLE_LIMIT)
    }
}

pub(super) fn over_role_limit_message(data: &RoleLimitData) -> String {
    let existing_roles_noun = pluralize("role", data.non_tulpje_roles() as isize, true);
    let members_noun = pluralize("system member", data.members as isize, true);
    let total_roles_noun = pluralize("role", data.roles_with_members() as isize, true);
    let over_limit = data.over_limit();

    format!(
        "### Error\n\
            This server currently has {existing_roles_noun}, \
            adding roles for {members_noun} would leave you with \
            {total_roles_noun} which is {over_limit} more than \
            discord's limit of {DISCORD_ROLE_LIMIT} roles",
    )
}

pub(super) fn near_role_limit_message(data: &RoleLimitData) -> String {
    let existing_roles_noun = pluralize("role", data.non_tulpje_roles() as isize, true);
    let members_noun = pluralize("system member", data.members as isize, true);
    let total_roles_noun = pluralize("role", data.roles_with_members() as isize, true);
    let remaining_noun = pluralize("remaining role", data.remaining() as isize, false);

    format!(
        "### Warning\n\
            This server currently has {existing_roles_noun}, \
            adding roles for {members_noun} \
            would be a total of {total_roles_noun} \
            which is close to discord's limit of {DISCORD_ROLE_LIMIT} roles,\
            leaving you with {remaining_noun}, is that okay?"
    )
}
