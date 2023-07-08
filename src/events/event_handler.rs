use crate::chatgpt_builder::ChatGptBuilder;
use poise::serenity_prelude::{ChannelType};
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
                if new_message.content == "ping" {
                    new_message.channel_id.say(&ctx, pingpong()).await?;
                }

                //----------chatgpt shenanigans
                {
                    //message in thread check
                    if new_message.channel_id.to_channel(&ctx).await?.guild().expect("Did not find guild").kind == ChannelType::PublicThread {

                        println!("message is in a thread");
                        println!("thread id: {}", new_message.channel_id);

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

                            let imput = &new_message.content[19..];
                            new_message.channel_id.create_public_thread(
                                &ctx,
                                new_message.id,
                                |b| b.name(&new_message.content[19..])).await?
                                .say(&ctx,
                                     {
                                         ChatGptBuilder::new(
                                             imput,
                                             &data,
                                             u64::from(new_message.id),
                                             &new_message.author,
                                             false
                                         )
                                             .await
                                             .expect("Cannot create chatgpt client, likely quota")
                                     }).await?;

                            println!("message id: {}", new_message.id);
                        }
                    }
                }

                //dev section (keep at the end so we can delete the message safely)
                {
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

        poise::Event::ThreadUpdate {thread: _} => {
            println!("thread update!")
        }

        poise::Event::ThreadCreate {thread: _} => {
            println!("thread created!")
        }

        poise::Event::ThreadDelete {thread} => {
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




