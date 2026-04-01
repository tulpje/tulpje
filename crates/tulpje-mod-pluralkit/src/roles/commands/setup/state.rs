#[derive(Debug, Default)]
pub(super) enum SetupStep {
    #[default]
    IfSetupShowConfig,
    CheckRoleLimit,
    MigrateRoles,
    RoleSuffix,
    CheckCharacterLimit,
    ConfirmationPrompt,
}

#[derive(Debug, Default)]
pub(super) struct SetupState {
    step: SetupStep,
    role_suffix: String,
}
