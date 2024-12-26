
use crate::osu::embed::CuchaEmbed;
use crate::osu::osu_client;
use crate::prelude::*;
use crate::scraping::scraping_builder;
use poise::CreateReply;
use rosu_v2::model::GameMode;
use tracing::{info, instrument, span, warn, Level};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

///get user info
#[poise::command(slash_command, prefix_command)]
pub async fn get_osu_user_info(
    ctx: Context<'_>,
    #[description = "User name"] user_name: String,
) -> Result<(), Error> {
    let data_arc = ctx.data().osu_data.clone();
    match osu_client::OsuClient::new(ctx)
        .await?
        .osu
        .user(user_name)
        .mode(GameMode::Osu)
        .await
    {
        Ok(user) => {
            let embed = CuchaEmbed::new_user_embed(&data_arc, user, None);
            let builder = CreateReply::default().embed(embed);
            ctx.send(builder)
                .await
                .expect("could not send the message.");
        }
        Err(_why) => {
            ctx.say(format!("El usuario no existe~")).await?;
        }
    }
    Ok(())
}

///gets osu recent score by username
#[poise::command(slash_command, prefix_command)]
pub async fn get_osu_recent_score_by_username(
    ctx: Context<'_>,
    #[description = "User name"] user_name: String,
) -> Result<(), Error> {
    let osu = osu_client::OsuClient::new(ctx).await?;
    let data_arc = ctx.data().osu_data.clone();
    match osu.get_recent_scores(&user_name).await {
        Ok(scores) => {
            if !scores.is_empty() {
                for score in scores {
                    let score_embed =
                        CuchaEmbed::new_score_embed(ctx.http(), &data_arc, score).await;
                    let builder = CreateReply::default().embed(score_embed);
                    ctx.send(builder)
                        .await
                        .expect("could not send the message.");
                }
            } else {
                ctx.say(format!("{} has not played recently e_e", user_name))
                    .await?;
            }
        }
        Err(_why) => {
            ctx.say(format!("El usuario no existe~")).await?;
        }
    }
    Ok(())
}

///gets osu best score by username
#[poise::command(slash_command, prefix_command)]
pub async fn get_osu_top_score_by_username(
    ctx: Context<'_>,
    #[description = "User name"] user_name: String,
) -> Result<(), Error> {
    let osu = osu_client::OsuClient::new(ctx).await?;
    let data_arc = ctx.data().osu_data.clone();
    match osu.get_best_scores(&user_name).await {
        Ok(scores) => {
            if !scores.is_empty() {
                for score in scores {
                    let score_embed =
                        CuchaEmbed::new_score_embed(ctx.http(), &data_arc, score).await;
                    let builder = CreateReply::default().embed(score_embed);
                    ctx.send(builder)
                        .await
                        .expect("could not send the message.");
                }
            } else {
                ctx.say(format!("{} has not played recently e_e", user_name))
                    .await?;
            }
        }
        Err(_why) => {
            ctx.say(format!("El usuario no existe~")).await?;
        }
    }
    Ok(())
}

///reverses your text!
#[poise::command(slash_command)]
pub async fn reverse(
    ctx: Context<'_>,
    #[description = "Message send"] message: String,
) -> Result<(), Error> {
    let response: String = Stack::<String>::invertir_frase(&message);
    ctx.say(response).await?;
    Ok(())
}

///Get the weekly world of warcraft mythic+ affix rotation.
#[poise::command(slash_command, prefix_command)]
pub async fn mythicweek(ctx: Context<'_>) -> Result<(), Error> {
    let mut response = "The current World of Warcraft Mythic+ affix are: ".to_string();
    response.push_str(&scraping_builder::ScraperBuilder::subcreation_weekly_affix().await?);
    ctx.say(response).await?;

    Ok(())
}

///About section.
#[poise::command(slash_command, prefix_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let mut output = String::new();
    let source_link = "https://github.com/Cucharoth/cuchabot_local.git";
    output.push_str("I'm CuchaBot, a discord bot. \nAsk me anything using this format:\n");
    output.push_str("hey cucha: tell me: [your_question]\n");
    output.push_str("---(You do no need the format inside threads!)\n");
    output.push_str("You can also try some commands using the '~' prefix\nSource: ");
    output.push_str(source_link);
    ctx.say(output).await?;
    Ok(())
}

///Shows all the commands.
#[poise::command(slash_command, prefix_command)]
pub async fn commands(ctx: Context<'_>) -> Result<(), Error> {
    let mut output = String::new();
    output.push_str("You can use commands with the '~' prefix\nYou can also ask CuchaBot to use them for you with:\nhey cucha, [command]\n\n");
    for var in ctx.framework().options.commands.iter() {
        let string = format!(
            "[{0}]: {1:width$} \n",
            var.name.to_string(),
            var.description.clone().unwrap().to_string(),
            width = 100 - var.name.to_string().len()
        );
        output.push_str(&string);
    }
    ctx.say(output).await?;

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
pub async fn tiburonsin(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("HU HA HA!").await?;
    Ok(())
}

///
#[poise::command(slash_command, prefix_command)]
#[instrument(level = "info", skip_all)]
pub async fn test(
    ctx: Context<'_>,
    #[description = "test"] test_text: String,
) -> Result<(), Error> {
    let data_arc = ctx.data().osu_data.clone();
    match test_text.as_str() {
        "score" => {
            println!("score embed test");
            let osu_score = osu_client::OsuClient::new(ctx)
                .await?
                .osu
                .score(3778790399)
                .await
                .unwrap();
            let score_embed = CuchaEmbed::new_score_embed(ctx.http(), &data_arc, osu_score).await;
            let builder = CreateReply::default().embed(score_embed);
            ctx.send(builder)
                .await
                .expect("could not send the message.");
        }
        "user" => {
            println!("user embed test");
            let user = osu_client::OsuClient::new(ctx)
                .await?
                .osu
                .user("xeamx")
                .mode(GameMode::Osu)
                .await
                .unwrap();
            let embed = CuchaEmbed::new_user_embed(&data_arc, user, None);
            let builder = CreateReply::default().embed(embed);
            ctx.send(builder)
                .await
                .expect("could not send the message.");
        }
        "log" => {
            let testo_span = span!(Level::INFO, "testo_span");
            let _testo_span_enter = testo_span.enter();
            testo().await;
            testo2().await;
            testo3().await;
        }
        "session" => {
            let session = {
                let mut mutex = data_arc.players_info.lock().unwrap();
                mutex.get_mut("Neme").unwrap().session.clone()
            };
            session.updated_user.statistics.clone().unwrap().pp = 20000.;
            let embed = CuchaEmbed::new_session_embed(&data_arc, &session);
            let builder = CreateReply::default().embed(embed);
            ctx.send(builder)
                .await
                .expect("could not send the message.");
        }
        _ => {
            ctx.say("¯\\_(ツ)_/¯").await?;
        }
    }
    Ok(())
}

#[instrument(level = "info", skip_all)]
async fn testo() {
    info!(name: "this is the testo1", target: "this_is_the_target", "this is a testo1");
}

#[instrument(level = "info", skip_all)]
async fn testo2() {
    warn!("this is a testo2");
}

#[instrument(level = "info", skip_all)]
async fn testo3() {
    info!("this is a testo3");
}
