use std::collections::HashMap;
use std::sync::Mutex;
use chatgpt::converse::Conversation;
use crate::prelude::*;
use poise::serenity_prelude;
use crate::chatgpt_builder::ChatGptBuilder;
use poise::serenity_prelude::MessageId;

//use crate::{Context, Error};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


pub struct Data {
    pub(crate) discord_thread_info: Mutex<HashMap<u64, Conversation>>,
    pub first_message: Mutex<String>,
}

///temporal reset for chatgpt
#[poise::command(slash_command, prefix_command)]
pub async fn reset(
    ctx: Context<'_>
) -> Result<(), Error> {
    ChatGptBuilder::reset();
    ctx.say("OK!, I've forgotten everything! @_@, bip bop").await?;
    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// TIBURONSIN: HUHAHA
#[poise::command(prefix_command)]
pub async fn tiburonsin(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("HU HA HA!").await?;
    Ok(())
}

//non poise implementation:
/*#[poise::command(prefix_command, slash_command)]
pub async fn test(
    ctx: Context<'_>,
    #[description = "a description"] arg: bool
) -> Result<(), Error> {
    if arg {
        ctx.say("testo").await?;
    }

    Ok(())
}*/



