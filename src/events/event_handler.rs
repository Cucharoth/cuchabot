use std::sync::Arc;

use crate::notifications::thread_handler::ThreadHandler;
use crate::ollama::chat::chat::OllamaChat;
use poise::serenity_prelude::FullEvent;
use serenity_prelude::ActivityData;
use serenity_prelude::CreateMessage;
use serenity_prelude::MessageFlags;
use tracing::error;
//use rand::Rng;
use crate::prelude::*;
type Error = Box<dyn std::error::Error + Send + Sync>;
type _Context<'a> = poise::Context<'a, Data, Error>;

use crate::Data;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        //todo: add events here:
        FullEvent::Message { new_message } => {
            if !new_message.author.bot {
                response_handler(ctx, new_message).await?;
                mention_handler(ctx, data, new_message).await?;
            }
        }

        FullEvent::PresenceUpdate { new_data: _ } => {
            //println!("{:#?}", new_data);
        }

        FullEvent::CacheReady { guilds: _ } => {
            ctx.set_activity(Some(ActivityData::playing("Osu!")));
            ThreadHandler::new(ctx, Arc::clone(&data.db)).await?;
        }

        _ => {}
    }
    Ok(())
}

// Main message catcher, put all the messages functions here
async fn response_handler(ctx: &serenity::Context, message: &Message) -> Result<(), Error> {
    pingpong(ctx, message).await?;
    tiburonsin(ctx, message).await?;
    pain(ctx, message).await?;
    Ok(())
}

async fn mention_handler(
    ctx: &serenity::Context,
    data: &Data,
    message: &Message,
) -> Result<(), Error> {
    if message.mentions_me(&ctx.http).await.unwrap() {
        let typing = message.channel_id.start_typing(&ctx.http);
        // this is currently responding to mentions messages
        // so we need to remove the mention part first
        let trimmed_input = message.content[23..].to_string();
        let bot_response = match OllamaChat::init_chat(&data.db, trimmed_input).await {
            Ok(bot_response) => bot_response,
            Err(why) => {
                error!("ollama error: {}", why);
                "cuchabot is asleep~".to_string()
            }
        };

        let builder = CreateMessage::new()
            .flags(MessageFlags::LOADING)
            .content(bot_response);
        typing.stop();
        message.channel_id.send_message(&ctx.http, builder).await?;
    };
    Ok(())
}

async fn tiburonsin(ctx: &serenity::Context, message: &Message) -> Result<(), Error> {
    if message.content.contains("tiburonsin") {
        message
            .channel_id
            .say(&ctx, String::from("HU HA HA!"))
            .await?;
    }
    Ok(())
}

async fn pingpong(ctx: &serenity::Context, message: &Message) -> Result<(), Error> {
    if message.content.contains("ping") {
        message.channel_id.say(&ctx, String::from("pong!")).await?;
    }
    Ok(())
}

async fn pain(ctx: &serenity::Context, message: &Message) -> Result<(), Error> {
    if message.content.contains("pain") {
        let rng = rand::random::<bool>();

        if message.content == "painguin" {
            let response = "https://i.imgur.com/Mb5EzYr.png".to_string();
            message.channel_id.say(&ctx, response).await?;
            return Ok(());
        }
        if message.content == "painpeko" {
            let response = "https://i.imgur.com/qSC3eMf.jpeg".to_string();
            message.channel_id.say(&ctx, response).await?;
            return Ok(());
        }

        let response = if rng {
            "https://i.imgur.com/qSC3eMf.jpeg".to_string() //pain_peko
        } else {
            "https://i.imgur.com/Mb5EzYr.png".to_string() //painguin
        };

        message.channel_id.say(&ctx, response).await?;
    }
    Ok(())
}
