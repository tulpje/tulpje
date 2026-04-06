use std::{num::ParseIntError, str::FromStr as _};

use pkrs_fork::model::Member;
use serde::{Deserialize, Serialize};
use tulpje_framework::{Error, color::Color};
use tulpje_lib::{
    context::Services,
    wizard::{WizardContext, WizardStep},
};
use uuid::Uuid;

use crate::{
    roles::{
        role_limit::RoleLimitData,
        setup::{
            custom_ids,
            view::{legacy_roles, near_role_limit, role_suffix},
        },
    },
    util::get_member_name,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct SetupState {
    pub(super) role_suffix: String,
    pub(super) member_data: Vec<(Uuid, String, Option<Color>)>,
    pub(super) legacy_roles: usize,
    pub(super) cleanup_legacy: bool,
}
impl SetupState {
    pub(super) fn with_member_data(members: &[Member]) -> Result<Self, ParseIntError> {
        let member_data: Vec<_> = members
            .iter()
            .map(|m| {
                Ok((
                    m.uuid,
                    get_member_name(m),
                    m.color.as_ref().map(|c| Color::from_str(c)).transpose()?,
                ))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            member_data,
            ..Default::default()
        })
    }
}

#[derive(Debug)]
pub(super) struct PromptNearRoleLimit {
    data: RoleLimitData,
}
impl PromptNearRoleLimit {
    pub(super) fn new(data: RoleLimitData) -> Self {
        Self { data }
    }
}

#[async_trait::async_trait]
impl WizardStep<SetupState> for PromptNearRoleLimit {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        near_role_limit::view(ctx, &self.data).await?;
        Ok(Some(SetupState {
            legacy_roles: self.data.legacy_member_roles,
            ..state
        }))
    }
}

#[derive(Debug)]
pub struct AcceptNearRoleLimit;
#[async_trait::async_trait]
impl WizardStep<SetupState> for AcceptNearRoleLimit {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        if state.legacy_roles > 0 {
            legacy_roles::view(ctx, state.legacy_roles).await?;
        } else {
            role_suffix::view(ctx, &state.role_suffix).await?;
        }
        Ok(Some(state))
    }
}

#[derive(Debug)]
pub struct DenyNearRoleLimit;
#[async_trait::async_trait]
impl WizardStep<SetupState> for DenyNearRoleLimit {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        _state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        near_role_limit::deny_view(ctx).await?;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct PromptLegacyRolesCleanup;
#[async_trait::async_trait]
impl WizardStep<SetupState> for PromptLegacyRolesCleanup {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        legacy_roles::view(ctx, state.legacy_roles).await?;
        Ok(Some(state))
    }
}

#[derive(Debug)]
pub struct AcceptLegacyRolesCleanup;
#[async_trait::async_trait]
impl WizardStep<SetupState> for AcceptLegacyRolesCleanup {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        role_suffix::view(ctx, &state.role_suffix).await?;
        Ok(Some(SetupState {
            cleanup_legacy: true,
            ..state
        }))
    }
}

#[derive(Debug)]
pub struct DenyLegacyRolesCleanup;
#[async_trait::async_trait]
impl WizardStep<SetupState> for DenyLegacyRolesCleanup {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        role_suffix::view(ctx, &state.role_suffix).await?;
        Ok(Some(SetupState {
            cleanup_legacy: false,
            ..state
        }))
    }
}

#[derive(Debug)]
pub struct PromptRoleSuffix;
#[async_trait::async_trait]
impl WizardStep<SetupState> for PromptRoleSuffix {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        role_suffix::view(ctx, &state.role_suffix).await?;
        Ok(Some(state))
    }
}

#[derive(Debug)]
pub struct AnswerRoleSuffix;
#[async_trait::async_trait]
impl WizardStep<SetupState> for AnswerRoleSuffix {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: SetupState,
    ) -> Result<Option<SetupState>, Error> {
        let role_suffix = ctx.get_form_field_text(custom_ids::INPUT_ROLE_SUFFIX)?;
        println!("role suffix: {role_suffix}");

        // NEXT STEP

        Ok(Some(SetupState {
            role_suffix,
            ..state
        }))
    }
}
