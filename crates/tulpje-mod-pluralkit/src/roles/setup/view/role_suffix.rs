use tulpje_framework::Error;
use tulpje_lib::{context::Services, wizard::WizardContext};
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::roles::setup::{components::prompt_role_suffix_component, custom_ids};

pub(crate) async fn view(ctx: &WizardContext<Services>, role_suffix: &str) -> Result<(), Error> {
    let response = InteractionResponse {
        kind: InteractionResponseType::Modal,
        data: Some(InteractionResponseData {
            title: Some("Role Suffix".into()),
            custom_id: Some(custom_ids::MODAL_ROLE_SUFFIX_SUBMIT.into()),
            components: Some(prompt_role_suffix_component(
                custom_ids::INPUT_ROLE_SUFFIX,
                role_suffix,
            )),

            ..Default::default()
        }),
    };

    ctx.client
        .interaction(ctx.application_id)
        .create_response(ctx.interaction_id, &ctx.interaction_token, &response)
        .await?;

    Ok(())
}
