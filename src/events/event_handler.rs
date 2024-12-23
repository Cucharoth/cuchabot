use std::sync::Arc;

use crate::data::osu_data::OsuData;
use crate::notifications::thread_handler::ThreadHandler;
use poise::serenity_prelude::FullEvent;
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
        FullEvent::GuildMemberAddition { new_member } => {
            println!("New member: {}", new_member.user.name);
        }

        FullEvent::Message{ new_message} => {
            if !new_message.author.bot {
                response_handler(ctx, new_message).await?;
            }
        }

        FullEvent::CacheReady { guilds: _ } => {
            let data_arc = Arc::new(OsuData::new(data));
            ThreadHandler::new(ctx, data_arc, data).await?;
        }

        FullEvent::ThreadUpdate {old: _, new: _} => {
            println!("thread update!")
        }

        FullEvent::ThreadCreate {thread: _} => {
            println!("thread created!")
        }

        FullEvent::ThreadDelete {thread, full_thread_data: _} => {
            println!("thread deleted id: {}", thread.id);
        }

        _ => {},

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

async fn tiburonsin(ctx: &serenity::Context, message: &Message) -> Result<(), Error>{
    if message.content.contains("tiburonsin"){
        message.channel_id.say(&ctx, String::from("HU HA HA!")).await?;
    }
    Ok(())
}

async fn pingpong(ctx: &serenity::Context, message: &Message) -> Result<(), Error>{
    if message.content.contains("ping"){
        message.channel_id.say(&ctx, String::from("pong!")).await?;
    }
    Ok(())
}

async fn pain(ctx: &serenity::Context, message: &Message) -> Result<(), Error>{
    if message.content.contains("pain"){
        let rng = rand::random::<bool>();
        
        if message.content == "painguin"{
            let response = "https://i.imgur.com/Mb5EzYr.png".to_string();
            message.channel_id.say(&ctx, response).await?;
            return Ok(());
        }
        if message.content == "painpeko"{
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


