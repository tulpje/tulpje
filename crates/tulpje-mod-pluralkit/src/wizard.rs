use serde::{Deserialize, Serialize};
use tulpje_framework::Error;
use tulpje_lib::{
    context::{CommandContext, Services},
    wizard::{WizardContext, WizardStep, start_wizard},
};
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle, TextDisplay};

pub(crate) const COMPONENT_CLEANUP_WIZARD_CONFIRM: &str = "CLEANUP_WIZARD/CONFIRM";
pub(crate) const COMPONENT_CLEANUP_WIZARD_DENY: &str = "CLEANUP_WIZARD/DENY";

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct CleanupWizardState {
    persistence_test: String,
}

#[derive(Debug)]
pub(crate) struct ConfirmStep;
#[async_trait::async_trait]
impl WizardStep<CleanupWizardState> for ConfirmStep {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        state: CleanupWizardState,
    ) -> Result<Option<CleanupWizardState>, Error> {
        ctx.update(&[TextDisplay {
            id: None,
            content: state.persistence_test,
        }
        .into()])
            .await?;

        Ok(None)
    }
}

#[derive(Debug)]
pub(crate) struct DenyStep;
#[async_trait::async_trait]
impl WizardStep<CleanupWizardState> for DenyStep {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        _state: CleanupWizardState,
    ) -> Result<Option<CleanupWizardState>, Error> {
        ctx.update(&[TextDisplay {
            id: None,
            content: "Nope".into(),
        }
        .into()])
            .await?;

        Ok(None)
    }
}

#[derive(Debug)]
struct AskStep;
#[async_trait::async_trait]
impl WizardStep<CleanupWizardState> for AskStep {
    async fn run(
        &self,
        ctx: &WizardContext<Services>,
        _state: CleanupWizardState,
    ) -> Result<Option<CleanupWizardState>, Error> {
        ctx.update(&[ActionRow {
            id: None,
            components: vec![
                Button {
                    id: None,
                    custom_id: Some(COMPONENT_CLEANUP_WIZARD_CONFIRM.into()),
                    disabled: false,
                    emoji: None,
                    label: Some("Confirm".into()),
                    url: None,
                    sku_id: None,
                    style: ButtonStyle::Success,
                }
                .into(),
                Button {
                    id: None,
                    custom_id: Some(COMPONENT_CLEANUP_WIZARD_DENY.into()),
                    disabled: false,
                    emoji: None,
                    label: Some("Deny".into()),
                    url: None,
                    sku_id: None,
                    style: ButtonStyle::Danger,
                }
                .into(),
            ],
        }
        .into()])
            .await?;

        Ok(Some(CleanupWizardState {
            persistence_test: "It saved!".into(),
        }))
    }
}

/// initial interaction, not present in the steps
pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    if ctx.event.guild_id.is_none() {
        unreachable!("command is guild_only");
    }

    ctx.defer_ephemeral()
        .await
        .map_err(|err| format!("failed to defer: {err}"))?;

    start_wizard(ctx, AskStep {}, None).await?;

    Ok(())
}
