use twilight_model::{
    channel::message::MessageFlags,
    id::{Id, marker::ChannelMarker},
};
use twilight_util::builder::message::{ContainerBuilder, TextDisplayBuilder};
use uuid::Uuid;

use crate::{Context, Error, color};

pub mod autocomplete_handler;
pub mod command_handler;
pub mod component_interaction_handler;
pub mod event_handler;
pub mod modal_handler;
pub mod task_handler;

async fn send_internal_handler_error<T: Clone + Send + Sync>(
    channel_id: Id<ChannelMarker>,
    uuid: Uuid,
    ctx: Context<T>,
) -> Result<(), Error> {
    ctx.client
        .create_message(channel_id)
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(&[ContainerBuilder::new()
            .accent_color(Some(*color::roles::RED))
            .component(
                // TODO: Better way to handle extra error info than, whatever this is
                TextDisplayBuilder::new(format!(
                    "### Internal Error\n{}\n**Error Code**\n```{}```",
                    std::env::var("TULPJE_EXTRA_ERROR_MESSAGE").unwrap_or_default(),
                    uuid
                ))
                .build(),
            )
            .build()
            .into()])
        .await?;

    Ok(())
}
