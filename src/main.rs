mod events;
mod commands;
mod slash_commands;
mod chatgpt;
mod scraping;
mod data;
mod osu;
mod notifications;

pub mod prelude{
    pub use shuttle_runtime::SecretStore;
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
    pub const OSU_SPAM_CHANNEL_ID: u64 = 1242292133965205597;
    pub const EMOJI_GUILD_ID: u64 = 1002656027088523286;
    //pub use shuttle_secrets::SecretStore;
    //pub use shuttle_poise::ShuttlePoise;
    //pub use anyhow::Context as _;
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use anyhow::Context as _;
use dotenv::{dotenv, var};
use shuttle_serenity::ShuttleSerenity;
use std::time::Duration;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use crate::prelude::*;
use crate::event_handler::event_handler;
use crate::slash_commands::slash_commands_handler::*;


#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    File::create("my_conversation.txt").expect("can't create log file");
    let discord_token= secret_store
        .get("TOKEN").context("")?;
    let osuinfo:(u64, String) = (secret_store.get("OSU_CLIENT_ID").context("")?.parse::<u64>().expect(""), secret_store.get("OSU_CLIENT_SECRET").context("")?);
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
                reverse(),
                get_osu_top_score_by_username(),
                get_osu_recent_score_by_username(),
                get_osu_user_info(),
                test()
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
        .setup(move |ctx, ready: &Ready, framework| {
            Box::pin(async move {
                println!("Connected as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    discord_thread_info: Mutex::new(HashMap::new()),
                    thread_info_as_bst: Mutex::new(BST::new()),
                    first_message: Mutex::new(String::new()),
                    osu_info: Mutex::new(osuinfo),
                    osu_pp: Mutex::new(HashMap::new()),
                    is_loop_running: AtomicBool::new(false),
                    secret_store: Mutex::new(secret_store),
                    cuchabot: Arc::new(ready.clone())
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




