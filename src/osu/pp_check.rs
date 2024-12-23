use std::{collections::HashMap, sync::Arc};
use crate::{data::osu_data::OsuData, prelude::*};

use poise::serenity_prelude::*;
use rosu_v2::{error::OsuError, model::{score::Score, user::UserExtended, GameMode}, Osu};
use tracing::{error, info, span, warn, Level};

use super::{dto::dto_osu_score::OsuScore, osu_client::OsuClient};


pub struct PpCheck;

impl PpCheck {
    pub async fn check_current_pp(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>) -> Result<(),OsuError>{
        let osu = OsuClient::new_from_thread(ctx, data).await?.osu;
        let mut users = vec![];
        {
            let data_mutex = data.osu_pp.lock().unwrap();
            let keys = data_mutex.keys();
            for user in keys {
                users.push(String::from(user));
            }
        };
        for current_username in users {
            let current_user = osu.user(current_username.clone()).mode(GameMode::Osu).await?;
            let old_pp;
            let pp_has_changed = {   
                let data_mutex = data.osu_pp.lock().unwrap();
                old_pp = data_mutex.get(&current_username).unwrap().0;
                let new_pp = current_user.clone().statistics.unwrap().pp;
                info!( "{}  \t | old pp: {} | new pp: {} |", current_username, old_pp, new_pp);
                new_pp - old_pp > 1.
            };
            info!("pp has changed: {}", pp_has_changed);
            if pp_has_changed {
                warn!("pp changed for {}!", current_username);
                let update_pp = span!(Level::INFO, "update_pp");
                let _update_pp_enter = update_pp.enter();
                Self::update_pp(ctx, data, current_user.clone(), old_pp).await;
                let new_score = Self::update_scores(data, current_user.clone(), &osu).await;
                Self::sends_message(ctx, data, new_score).await;
            }
        }
        Ok(())
    }

    pub async fn setup(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>) -> Result<(),OsuError> {
        info!("started osu score setup");
        let osu = OsuClient::new_from_thread(ctx, &data).await?.osu;
        let xeamx = osu.user("xeamx").mode(GameMode::Osu).await.expect("user not found");
        let xeamx_scores = osu.user_scores(xeamx.user_id).best().limit(100).mode(GameMode::Osu).await?;
        let neme = osu.user("Neme").mode(GameMode::Osu).await.expect("user not found");
        let neme_scores = osu.user_scores(neme.user_id).best().limit(100).mode(GameMode::Osu).await?;
        let mut xeamx_map = HashMap::new();
        let mut neme_map = HashMap::new();
        for score in xeamx_scores {
            xeamx_map.insert(score.map_id, OsuScore::new(score));
        }
        for score in neme_scores {
            neme_map.insert(score.map_id, OsuScore::new(score));
        }
        {
            let mut osu_pp = data.osu_pp.lock().unwrap();
            osu_pp.insert("xeamx".to_string(), (xeamx.clone().statistics.expect("not found").pp, xeamx_map));
            osu_pp.insert("Neme".to_string(), (neme.clone().statistics.expect("not found").pp, neme_map));
        }
        info!("osu setup done");
        info!("{} {}", xeamx.clone().statistics.expect("not found").pp, neme.statistics.expect("not found").pp);
        Ok(())
    }

    async fn update_scores(data: &Arc<OsuData>, current_user: UserExtended, osu: &Osu) -> Score{
        let new_scores = osu.user_scores(&current_user.username.to_string()).best().limit(100).mode(GameMode::Osu).await.expect("");
        let mut new_map = HashMap::new();
        for score in new_scores.clone() {
            new_map.insert(score.map_id, OsuScore::new(score));
        }
        info!("Trying to update scores.");
        {
            let mut data = data.osu_pp.lock().unwrap();
            data.get_mut(&current_user.username.to_string()).unwrap().1 = new_map;
        }
        info!("Done updating scores.");
        new_scores.iter().max_by_key(|x| x.ended_at).expect("no score found").clone()
    }

    async fn update_pp(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>, current_user: UserExtended, old_pp: f32) {
        info!("Started update pp sequence.");
        //OsuScore::ember_user(ctx, data, current_user.clone(), dif_pp).await;
        // Creates discord embed and sends it
        let user_embed = OsuScore::get_embed_user(data, current_user.clone(), Some(old_pp));
        let builder = CreateMessage::new().embed(user_embed);
        ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, builder).await.expect("could not send message");

        let current_pp = current_user.clone().statistics.unwrap().pp;
        info!("Cloned current pp");
        {
            match data.osu_pp.lock() {
                Ok(mut data) => {
                    match data.get_mut(&current_user.username.to_string()) {
                    Some(tuple) => tuple.0 = current_pp,
                    None => error!("The name was not found in the data tuple"),
                }},
                Err(why) => println!("{}", why),
            }
        }
        info!("Updated data with the new pp score");
    }

    async fn sends_message(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>, score: Score) {
        let embed_score = OsuScore::get_embed_score(&ctx.http, data, score.clone()).await;
        let builder = CreateMessage::new().embed(embed_score);
        ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, builder).await.expect("could not send message");
        if score.is_perfect_combo {
            let perfect_builder = CreateMessage::new().embed(CreateEmbed::new().image(PERFECT_IMAGE));
            ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, perfect_builder).await.expect("could not send message");
        } else if score.statistics.combo_break == 0 && score.statistics.miss == 0 {
            let fc_builder = CreateMessage::new().embed(CreateEmbed::new().image(FULL_COMBO));
            ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, fc_builder).await.expect("could not send message");
        };
    }
}