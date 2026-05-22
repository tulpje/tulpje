use base64::{Engine as _, prelude::BASE64_STANDARD};
use futures_util::StreamExt as _;
use tulpje_common::version;
use tulpje_lib::responses;
use twilight_http::Client;

use tulpje_framework::Error;
use tulpje_lib::context::CommandContext;
use twilight_model::guild::{Guild, PremiumTier};
use twilight_model::id::Id;
use twilight_model::id::marker::{EmojiMarker, GuildMarker};

use crate::db::Emoji;
use crate::shared::parse_emojis_from_string;

const EMOJI_CLONE_LIMIT: usize = 10;
async fn handle_emoji_clone_limit_error(ctx: &CommandContext) -> Result<(), Error> {
    responses::error(
        ctx,
        &format!("### ERROR\ncan't add more than {EMOJI_CLONE_LIMIT} emotes at once"),
    )
    .await
}

fn guild_emoji_limit(guild: &Guild) -> usize {
    match guild.premium_tier {
        PremiumTier::None => 50,
        PremiumTier::Tier1 => 100,
        PremiumTier::Tier2 => 150,
        PremiumTier::Tier3 => 250,
        _ => {
            tracing::warn!("unknown premium tier {:?}", guild.premium_tier);
            50
        }
    }
}

fn count_emojis(emojis: &[Emoji]) -> (usize, usize) {
    emojis
        .iter()
        .fold((0, 0), |(num_normal, num_animated), emoji| {
            if emoji.animated {
                (num_normal, num_animated + 1)
            } else {
                (num_normal + 1, num_animated)
            }
        })
}

/// checks if guild has space for emojis, and communicates to the user
/// expects calling function to return early if false is returned
async fn handle_emoji_limits(
    ctx: &CommandContext,
    guild: &Guild,
    emojis: &[Emoji],
) -> Result<bool, Error> {
    let (guild_normal, guild_animated) = count_emojis(
        &guild
            .emojis
            .iter()
            .map(|emoji| Emoji::from_twilight(emoji.clone(), guild.id))
            .collect::<Vec<_>>(),
    );
    let (new_normal, new_animated) = count_emojis(emojis);
    let emoji_limit = guild_emoji_limit(guild);
    let normal_over_limit = guild_normal + new_normal > emoji_limit;
    let animated_over_limit = guild_animated + new_animated > emoji_limit;

    if !normal_over_limit && !animated_over_limit {
        return Ok(true);
    }

    let mut text = String::from("### Error");
    if normal_over_limit {
        let over_limit = guild_normal + new_normal - emoji_limit;
        text.push_str(&format!("\nAdding {new_normal} **normal** emojis would exceed the limit of **{emoji_limit}** of this server by **{over_limit}**"));
    }
    if animated_over_limit {
        let over_limit = guild_animated + new_animated - emoji_limit;
        text.push_str(&format!("\nAdding {new_animated} **animated** emojis would exceed the limit of **{emoji_limit}** of this server by **{over_limit}**"));
    }

    responses::error(ctx, &text).await?;
    Ok(false)
}

// requires CREATE_GUILD_EXPRESSIONS permission
pub(crate) async fn command(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    // defer, we might be a while
    ctx.defer().await?;

    let emojis = parse_emojis_from_string(
        Id::<GuildMarker>::new(1), /* DUMMY */
        &ctx.get_arg_string("emoji")?,
    );
    if emojis.is_empty() {
        responses::error(&ctx, "### ERROR\nNo emojis found in command argument").await?;
        return Ok(());
    }

    if !handle_emoji_limits(&ctx, &guild, &emojis).await? {
        return Ok(());
    }

    if let Some(new_name) = ctx.get_arg_string_optional("new_name")? {
        // add single emote with new_name
        if emojis.len() > 1 {
            responses::error(
                &ctx,
                "### Error\ncan't add more than one emote at a time when specifying name",
            )
            .await?;
            return Ok(());
        }

        match clone_emoji(&ctx.client, guild.id, emojis.first().unwrap(), &new_name).await {
            Ok(emoji) => responses::success(&ctx, &format!("### Success\nAdded: {emoji}")).await?,
            Err(err) => {
                responses::error(
                    &ctx,
                    &format!("### Error\nError while cloning emoji\n```{err}```"),
                )
                .await?;
            }
        }

        return Ok(());
    } else if emojis.len() > EMOJI_CLONE_LIMIT {
        handle_emoji_clone_limit_error(&ctx).await?;
        return Ok(());
    }

    // add multiple emotes
    let prefix = ctx.get_arg_string_optional("prefix")?;
    clone_emojis(&ctx, guild.id, prefix, emojis).await?;
    Ok(())
}

// requires CREATE_GUILD_EXPRESSIONS permission
pub(crate) async fn context_command(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    // defer, we might be a while
    ctx.defer().await?;

    let Some(resolved) = &ctx.command.resolved else {
        return Err("no resolved data for context command".into());
    };
    let Some(message) = resolved.messages.values().next() else {
        return Err("no message for context command".into());
    };

    let emojis =
        parse_emojis_from_string(Id::<GuildMarker>::new(1) /* DUMMY */, &message.content);
    if emojis.is_empty() {
        responses::error(&ctx, "### ERROR\nNo emojis found in message").await?;
        return Ok(());
    }
    if emojis.len() > EMOJI_CLONE_LIMIT {
        handle_emoji_clone_limit_error(&ctx).await?;
        return Ok(());
    }

    clone_emojis(&ctx, guild.id, None, emojis).await?;
    Ok(())
}

async fn download_emoji(id: Id<EmojiMarker>, animated: bool) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(format!("Tulpje {}", version!()))
        .build()?;

    client
        .get(format!("https://cdn.discordapp.com/emojis/{id}.webp"))
        .query(&[("animated", animated)])
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await
        .map(|b| {
            // convert to a data uri
            format!("data:image/webp;base64,{}", BASE64_STANDARD.encode(b),)
        })
}

async fn clone_emojis(
    ctx: &CommandContext,
    guild_id: Id<GuildMarker>,
    prefix: Option<String>,
    emojis: Vec<Emoji>,
) -> Result<(), Error> {
    let prefix = prefix.unwrap_or_default();

    // what a fucken mess to have async map, but it works :)
    let emoji_results: Vec<Result<Emoji, EmojiError>> =
        futures_util::stream::iter(emojis.into_iter().map(async |e| {
            clone_emoji(&ctx.client, guild_id, &e, &format!("{}{}", &prefix, e.name)).await
        }))
        .buffered(1)
        .collect()
        .await;

    let emojis_added: Vec<String> = emoji_results
        .iter()
        .filter_map(|r| r.as_ref().ok().map(ToString::to_string))
        .collect();

    let emoji_errors: Vec<String> = emoji_results
        .iter()
        .filter_map(|r| match r {
            Ok(_) => None,
            Err(e) => {
                tracing::warn!("{e}");
                Some(e.to_string())
            }
        })
        .collect();

    // response if everything succeeded
    if emoji_errors.is_empty() {
        responses::success(
            ctx,
            &format!(
                "### Success\n\
                **The following emojis were added**\n\
                {}",
                emojis_added.join("")
            ),
        )
        .await?;
        return Ok(());
    }

    // response if everything failed
    if emojis_added.is_empty() {
        responses::error(
            ctx,
            &format!(
                "### Error\n**While adding emojis the following errors occured**\n```{}```",
                emoji_errors.join("\n\n")
            ),
        )
        .await?;
        return Ok(());
    }

    // response if mixed results
    responses::warning(
        ctx,
        &format!(
            "### Completed\n\
            **The following emojis were added**\n\
            {}\n\n\
            **While trying to clone emojis the following errors occured**\n\
            ```{}```",
            emojis_added.join(""),
            emoji_errors.join("\n\n"),
        ),
    )
    .await?;
    Ok(())
}

async fn clone_emoji(
    client: &Client,
    guild_id: Id<GuildMarker>,
    emoji: &Emoji,
    new_name: &str,
) -> Result<Emoji, EmojiError> {
    let emoji_data_uri = download_emoji(*emoji.id, emoji.animated)
        .await
        .map_err(|err| EmojiError::Download(emoji.clone(), err))?;

    let new_emoji = client
        .create_emoji(guild_id, new_name, &emoji_data_uri)
        .await
        .map_err(|e| EmojiError::Create(emoji.clone(), e))?
        .model()
        .await
        .map_err(|e| EmojiError::Other(emoji.clone(), e.into()))?;

    Ok(Emoji::from_twilight(new_emoji, guild_id))
}

pub(crate) enum EmojiError {
    Other(Emoji, Error),
    Download(Emoji, reqwest::Error),
    Create(Emoji, twilight_http::Error),
}

impl EmojiError {
    pub(crate) fn as_str(&self) -> String {
        match self {
            Self::Download(emoji, err) => {
                format!("error downloading emoji ({}): {}", emoji.name, err)
            }
            Self::Create(emoji, err) => {
                format!("error creating emoji ({}): {}", emoji.name, err)
            }
            Self::Other(emoji, err) => format!("unkown error ({}): {}", emoji.name, err),
        }
    }
}

impl std::fmt::Display for EmojiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}
impl std::fmt::Debug for EmojiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for EmojiError {}
