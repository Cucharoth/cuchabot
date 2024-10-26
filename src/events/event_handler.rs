use std::sync::Arc;

use crate::chatgpt_builder::ChatGptBuilder;
use crate::data::osu_data::OsuData;
use crate::notifications::thread_handler::ThreadHandler;
use poise::framework;
use poise::serenity_prelude::CreateThread;
use poise::serenity_prelude::{ChannelType, FullEvent};
//use rand::Rng;
use crate::prelude::*;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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

                //----------chatgpt shenanigans
                {
                    //message in thread check
                    if new_message.channel_id.to_channel(&ctx).await?
                        .guild()
                        .expect("Did not find guild")
                        .kind == ChannelType::PublicThread {

                        //lock workaround
                        let mut conversation_exist = false;
                        {
                            let key_value = u64::from(new_message.channel_id);
                            if let Some(_value) = data.thread_info_as_bst.lock().unwrap().get_by_key(key_value) {
                                conversation_exist = true;
                            }
                        }
                        if conversation_exist {
                            let imput = &new_message.content;
                            if let Ok(message) = ChatGptBuilder::new(
                                imput,
                                &data,
                                u64::from(new_message.channel_id),
                                &new_message.author,
                                true
                            )
                                .await {
                                new_message.channel_id.say(&ctx, message).await?;
                            }
                        }
                    }


                    //bot message filter and thread builder
                    if new_message.content.len() > 25 {
                        let mensage_cropped = &new_message.content[11..19];
                        if mensage_cropped == "tell me:" {
                            {
                                let mut string = data.first_message.lock().unwrap();
                                string.push_str(&new_message.content);
                            }

                            let input = &new_message.content[19..];
                            new_message.channel_id.create_thread_from_message(
                                &ctx,
                                new_message.id,
                                CreateThread::new(input)
                            ).await?
                                    .say(&ctx,
                                        {
                                            ChatGptBuilder::new(
                                                input,
                                                &data,
                                                u64::from(new_message.id),
                                                &new_message.author,
                                                false
                                            )
                                                .await
                                                .expect("Cannot create chatgpt client, likely quota")
                                        }).await?;
                            }
                    }
                }

                //dev section (keep at the end so we can delete the message safely)
                {
                    // set your ID here
                    let dev_id = 218661386950148096;

                    if new_message.author.id == dev_id {
                        // threads logs:
                        if new_message.content == "-threads log" {
                            ChatGptBuilder::read_logs();
                            new_message.delete(&ctx).await?;
                        }
                    }
                }
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
            {
                let key = thread.id; //threads ID correspond to the original message ID
                let mut bst = data.thread_info_as_bst.lock().unwrap();
                bst.delete_by_key(u64::from(key));
            }
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


