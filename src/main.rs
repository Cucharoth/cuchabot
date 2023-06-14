mod events;
mod commands;
mod slash_commands;
mod chatgpt;

mod prelude{
    //pub use serenity::async_trait;
    //pub use serenity::prelude::*;
    //pub use serenity::model::channel::Message;
    //pub use serenity::framework::standard::macros::{command, group};
    //pub use serenity::framework::standard::{StandardFramework, CommandResult};
    //pub use serenity::model::gateway::Ready;
    //pub use serenity::utils::Content;
    pub use crate::events::*;
    pub use crate::chatgpt::*;
    pub use poise::serenity_prelude;
    pub use poise::serenity_prelude as serenity;
    pub use poise::serenity_prelude::{EventHandler, Message, Ready};
    pub use poise::async_trait;
    pub use dotenv::{dotenv, var};
    pub use std::env;
    pub use std::sync::Mutex;
    pub use std::collections::HashMap;
    //pub use shuttle_secrets::SecretStore;
    //pub use shuttle_poise::ShuttlePoise;
    //pub use anyhow::Context as _;

}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use dotenv::{dotenv, var};
use std::time::Duration;
use poise::serenity_prelude::{GatewayIntents};
//use shuttle_poise::ShuttlePoise;
//fix here in case Error's wrong
use prelude::*;
use crate::event_handler::event_handler;
use crate::slash_commands::slash_commands_handler::{age, Data, reset, tiburonsin};
//use crate::commands::commands_handler::GENERAL_GROUP;
//use crate::slash_commands::slash_commands_handler::*;


#[tokio::main]
async fn main() {
    chatgpt_builder::ChatGptBuilder::reset();
    dotenv().ok();
    poise::Framework::builder()
        .token(env::var("TOKEN").expect("Missing 'TOKEN' env"))
        .options(poise::FrameworkOptions {
            commands: vec![
                //todo: commands here
                age(),
                tiburonsin(),
                reset()
            ],
            event_handler: |ctx, event, framework, data|
                Box::pin(event_handler(ctx, event, framework, data)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
                additional_prefixes: vec![
                    poise::Prefix::Literal("hey cucha,"),
                    poise::Prefix::Literal("hey cucha"),
                    poise::Prefix::Literal("hey cucha,"),
                    poise::Prefix::Literal("hey cucha giv"),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Connected as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    discord_thread_info: Mutex::new(HashMap::new()),
                    first_message: Mutex::new(String::new()),
                })
            })
        })
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,)
        .run()
        .await
        .unwrap();

}

    //non poise implementation:
    /*
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occured while running the client: {:?}", why);
    }*/


/*async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}*/



