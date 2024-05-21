use std::{collections::HashMap, env::current_exe, sync::{Arc, MutexGuard}};
use crate::{data::osu_data::OsuData, prelude::*};

use poise::serenity_prelude::*;
use rosu_v2::{error::OsuError, model::{user::{User, UserExtended}, GameMode}, Osu};

use crate::{data, Data};

use super::{dto::dto_osu_score::OsuScore, osu_client::OsuClient};


pub struct PpCheck;

impl PpCheck {
    pub async fn check_current_pp(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>) -> Result<(),OsuError>{
        let mut message = String::new();
        let osu = OsuClient::new_from_thread(ctx, data).await?.osu;
        let xeamx_current = osu.user("xeamx").mode(GameMode::Osu).await?;
        let neme_current = osu.user("neme").mode(GameMode::Osu).await?;
        let pp_has_changed = {   
            let data_mutex = data.osu_pp.lock().unwrap();
            data_mutex.get("xeamx").unwrap().0 != xeamx_current.clone().statistics.unwrap().pp
        };
        if pp_has_changed {
            Self::update_pp(ctx, data, xeamx_current.clone()).await;
            let new_score = Self::update_scores(data, xeamx_current.clone(), &osu).await;
            Self::print_new_score(ctx, xeamx_current.clone(), new_score).await;
        }
        let pp_has_changed = {   
            let data_mutex = data.osu_pp.lock().unwrap();
            data_mutex.get("neme").unwrap().0 != neme_current.clone().statistics.unwrap().pp
        };
        if pp_has_changed {
            Self::update_pp(ctx, data, neme_current.clone()).await;
            let new_score = Self::update_scores(data, neme_current.clone(), &osu).await;
            Self::print_new_score(ctx, neme_current.clone(), new_score).await;
        }
        Ok(())
    }

    pub async fn setup(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>) -> Result<(),OsuError> {
        println!("started osu score setup");
        let osu = OsuClient::new_from_thread(ctx, &data).await?.osu;
        let xeamx = osu.user("xeamx").mode(GameMode::Osu).await.expect("user not found");
        let xeamx_scores = osu.user_scores(xeamx.user_id).best().limit(100).mode(GameMode::Osu).await?;
        let neme = osu.user("neme").mode(GameMode::Osu).await.expect("user not found");
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
            osu_pp.insert("xeamx".to_string(), (xeamx.statistics.expect("not found").pp, xeamx_map));
            osu_pp.insert("neme".to_string(), (neme.statistics.expect("not found").pp, neme_map));
        }
        println!("osu setup done");
        Ok(())
    }

    async fn print_new_score(ctx: &poise::serenity_prelude::Context, current_user: UserExtended, score: OsuScore) {
        let message = format!("new score: \n {:#?}", score);
        let builder = CreateMessage::new().content(message);
        ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, builder).await.expect("wrong channel id");
    }

    async fn update_scores(data: &Arc<OsuData>, current_user: UserExtended, osu: &Osu) -> OsuScore{
        let new_scores = osu.user_scores(&current_user.username.to_string()).best().limit(100).mode(GameMode::Osu).await.expect("");
        let mut new_map = HashMap::new();
        for score in new_scores.clone() {
            new_map.insert(score.map_id, OsuScore::new(score));
        }
        {
            data.osu_pp.lock().unwrap().get_mut(&current_user.username.to_string()).unwrap().1 = new_map;
        }
        OsuScore::new(new_scores.iter().max_by_key(|x| x.ended_at).expect("no score found").clone())
    }

    async fn update_pp(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>, current_user: UserExtended) {
        let message_new_score = format!("{} got a new score!", current_user.username);
        let current_pp = current_user.clone().statistics.unwrap().pp;
        {
            data.osu_pp.lock().unwrap().get_mut(&current_user.username.to_string()).unwrap().0 = current_pp; 
        }
        let message = format!("{} {} current pp: {}", message_new_score, current_user.username, current_pp);
        let builder = CreateMessage::new().content(message);
        ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, builder).await.expect("wrong channel id");
    }
}