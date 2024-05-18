mod events;
mod commands;
mod slash_commands;
mod chatgpt;
mod scraping;
mod data;

pub mod prelude{
    pub use crate::scraping::*;
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
    pub use crate::data::data::*;
    pub use crate::data::bst::BST;
    pub use crate::data::stack::*;
    //pub use shuttle_secrets::SecretStore;
    //pub use shuttle_poise::ShuttlePoise;
    //pub use anyhow::Context as _;
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use std::fs::File;
use std::sync::Arc;
use anyhow::Context as _;
use dotenv::{dotenv, var};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::time::Duration;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use crate::prelude::*;
use crate::event_handler::event_handler;
use crate::slash_commands::slash_commands_handler::{about, age, reset, tiburonsin, commands, mythicweek, reverse};


#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    File::create("my_conversation.txt").expect("can't create log file");
    let discord_token= secret_store
        .get("TOKEN").context("")?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                //todo: commands here
                age(),
                tiburonsin(),
                reset(),
                about(),
                commands(),
                mythicweek(),
                reverse()
            ],
            event_handler: |ctx, event, framework, data|
                Box::pin(event_handler(ctx, event, framework, data)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(Duration::from_secs(3600)))),
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
                    thread_info_as_bst: Mutex::new(BST::new()),
                    first_message: Mutex::new(String::new()),
                })
            })
        }).build();

        let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
            .framework(framework)
            .await
            .map_err(shuttle_runtime::CustomError::new)?;

        
        //.intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,)
        Ok(client.into())

}




