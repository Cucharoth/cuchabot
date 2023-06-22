use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::Builder;
use chatgpt::converse::Conversation;
use crate::prelude::*;
use poise::serenity_prelude;
use poise::serenity_prelude::EntityType::Str;
use crate::chatgpt_builder::ChatGptBuilder;
use poise::serenity_prelude::MessageId;


//use crate::{Context, Error};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


pub struct Data {
    pub(crate) discord_thread_info: Mutex<HashMap<u64, Conversation>>,
    pub first_message: Mutex<String>,
}

///Get the weekly world of warcraft mythic+ affix rotation
#[poise::command(slash_command, prefix_command)]
pub async fn mythicweek(
    ctx: Context<'_>
) -> Result<(), Error> {
    let response = scraping_builder::ScraperBuiler::subcreation_weekly_affix().await?;
    ctx.say(response).await?;

    Ok(())
}

///About section
#[poise::command(slash_command, prefix_command)]
pub async fn about(
    ctx: Context<'_>
) -> Result<(), Error> {
    let mut output = String::new();
    let source_link = "https://github.com/Cucharoth/cuchabot_local.git";
    output.push_str("I'm CuchaBot, a discord bot. \nAsk me anything using this format:\n");
    output.push_str("hey cucha: tell me: [your_question]\n");
    output.push_str("You can also try some commands using the '~' prefix\nSource: ");
    output.push_str(source_link);
    ctx.say(output).await?;
    Ok(())
}

///Shows all the commands
#[poise::command(slash_command, prefix_command)]
pub async fn commands(
    ctx: Context<'_>
) -> Result<(), Error> {
    let mut output = String::new();
    output.push_str("You can use commands with the '~' prefix\nYou can also ask CuchaBot to use them for you with:\nhey cucha, [command]\n\n");
    for var in ctx.framework().options.commands.iter() {
        let string = format!("[{0}]: {1:width$} \n", var.name.to_string(), var.description.clone().unwrap().to_string(), width = 100 - var.name.to_string().len());
        print!("{}", string);
        output.push_str(&string);
    }
    ctx.say(output).await?;

    Ok(())
}

///Clears all the "memory" data for Chatgpt module
#[poise::command(slash_command, prefix_command)]
pub async fn reset(
    ctx: Context<'_>
) -> Result<(), Error> {
    //ChatGptBuilder::reset();
    {
        let mut hash_map = ctx.data().discord_thread_info.lock().unwrap();
        hash_map.clear()
    }
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



