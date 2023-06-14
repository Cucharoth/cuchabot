use std::hash::Hasher;
use crate::chatgpt_builder::ChatGptBuilder;
use std::io::Write;
use std::ops::Deref;
use std::ptr::hash;
use std::sync::mpsc::channel;
use chatgpt::converse::Conversation;
use poise::futures_util::future::err;
use poise::serenity_prelude;
use poise::serenity_prelude::{ChannelType, CreateThread, MessageId};
use poise::serenity_prelude::Mention::Channel;
use crate::prelude::*;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use crate::Data;


pub async fn event_handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        //todo: add events here:
        poise::Event::GuildMemberAddition { new_member } => {
            println!("New member: {}", new_member.user.name);
        }
        poise::Event::Message{ new_message} => {
            if !new_message.author.bot {
                if new_message.content == "!ping" {
                    new_message.channel_id.say(&ctx, pingpong()).await?;
                }

                //----------chatgpt shenanigans
                //message in thread check
                if new_message.channel_id.to_channel(&ctx).await?.guild().expect("Did not find guild").kind == ChannelType::PublicThread {
                    let mut is_first_message = false;
                    {
                        let first_message = data.first_message.lock().unwrap().to_string();
                        if new_message.content == first_message {
                            println!("is first message");
                            is_first_message = true;
                        }
                    }

                    println!("message is in a thread");
                    println!("thread id: {}", new_message.channel_id);
                    {
                        for (key, value) in data.discord_thread_info.lock().unwrap().iter() {
                            println!("channel id: {}", key);
                        }
                        let first_message = data.first_message.lock().unwrap().to_string();
                        println!("first message is: {}", first_message);
                    }

                    if !is_first_message {
                        //lock workaround
                        let mut conversation_exist = false;
                        {
                            let key_value = u64::from(new_message.channel_id);
                            if let Some(value) = data.discord_thread_info.lock().unwrap().get(&key_value) {
                                conversation_exist = true;
                            }
                        }
                        if conversation_exist {
                            let imput = &new_message.content;
                            if let Ok(message) = ChatGptBuilder::new(imput, &data, u64::from(new_message.channel_id), true).await {
                                new_message.channel_id.say(&ctx, message).await?;
                            }
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

                        let imput = &new_message.content[19..];
                        new_message.channel_id.create_public_thread(
                            &ctx,
                            new_message.id,
                            |b| b.name(&new_message.content[19..])).await?
                            .say(&ctx,
                                 {
                                     ChatGptBuilder::new(imput, &data, u64::from(new_message.id), false).await?
                                 }).await?;

                        println!("message id: {}", new_message.id);

                        //let chat_gpt = chatgpt_builder::ChatGptBuilder::new(imput).await;
                        /*if let Ok(mensage) = ChatGptBuilder::new(imput).await {
                        new_message.channel_id.say(&ctx, mensage).await?;
                    }*/
                    }
                }
            }
        }

        poise::Event::ThreadUpdate {thread} => {
            println!("thread update!")
        }

        poise::Event::ThreadCreate {thread} => {
            println!("thread created!")
        }

        poise::Event::ThreadDelete {thread} => {
            println!("thread deleted id: {}", thread.id);
            {
                let mut hash_map = data.discord_thread_info.lock().unwrap();
                hash_map.remove(&u64::from(thread.id));
            }
        }

        _ => {},




    }
    Ok(())

}

/*fn asking_chat_gpt(imput: &str) -> String {
    let respuesta = ChatGptBuilder::new(imput).expect("Error al crear chatGPT cliente");
    respuesta
}*/

fn pingpong() -> String{
let respuesta = String::from("pong!");
respuesta
}

//non poise implementation:
/*pub struct Handler;

impl Handler {
//todo: funciones aqui
fn pingpong(ctx: &Context, msg: Message) -> String{
let respuesta = String::from("pong!");
respuesta
}
}

#[async_trait]
impl EventHandler for Handler {
//todo: events here
async fn message(&self, ctx: Context, msg: Message) {
//todo: implementar reaccion a mensajes aqui:

//todo: como implementar funciones, await solo puede usarse en este scope --- half solved
if msg.content == "!ping" {
if let Err(why) = msg.channel_id.say(&ctx.http, Self::pingpong(&ctx, msg)).await {
println!("Error sending message: {:?}", why);
}
}


}

async fn ready(&self, _: Context, ready: Ready) {
println!("Connected as {}", ready.user.name);
}

}*/




