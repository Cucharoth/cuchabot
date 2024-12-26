use crate::{
    data::osu_data::{OsuData, PlayerInfo, PpInfo, SessionInfo},
    osu::{osu_client::OsuClient, user_activity::UserActivity},
    prelude::*,
};
use std::{collections::HashMap, sync::Arc};

use poise::serenity_prelude::*;
use rosu_v2::{
    error::OsuError,
    model::{score::Score, user::UserExtended, GameMode},
    Osu,
};
use tracing::{error, info, instrument, span, warn, Level};

use super::{dto::dto_osu_score::OsuScore, embed::CuchaEmbed};

pub struct PpCheck;

impl PpCheck {
    #[instrument(level = "info", skip_all)]
    pub async fn check_current_pp(
        ctx: &poise::serenity_prelude::Context,
        data: &Arc<OsuData>,
        osu: &Osu,
        current_user: &UserExtended,
    ) -> Result<(), OsuError> {
        let current_username = &current_user.username.to_string();
        let old_pp;
        let pp_has_changed = {
            let data_mutex = data.players_info.lock().unwrap();
            old_pp = data_mutex.get(current_username).unwrap().pp_info.current_pp;
            let new_pp = current_user.clone().statistics.unwrap().pp;
            info!(
                "{}  \t | old pp: {} | new pp: {} |",
                current_username, old_pp, new_pp
            );
            new_pp - old_pp > 1.
        };
        info!("pp has changed: {}", pp_has_changed);
        if pp_has_changed {
            warn!("pp changed for {}!", current_username);
            let update_pp = span!(Level::INFO, "update_pp");
            let _update_pp_enter = update_pp.enter();
            Self::update_pp(ctx, data, current_user.clone(), old_pp).await;
            UserActivity::update_session(ctx, data, current_user);
            let new_score = Self::update_scores(data, current_user.clone(), &osu).await;
            Self::sends_message(ctx, data, new_score).await;
        }
        Ok(())
    }

    #[instrument(level = "info", skip_all)]
    pub async fn setup(
        ctx: &poise::serenity_prelude::Context,
        data: &Arc<OsuData>,
    ) -> Result<(), OsuError> {
        info!("started osu score setup");
        let osu = OsuClient::new_from_thread(ctx, &data).await?.osu;
        let xeamx = osu
            .user("xeamx")
            .mode(GameMode::Osu)
            .await
            .expect("user not found");
        let xeamx_scores = osu
            .user_scores(xeamx.user_id)
            .best()
            .limit(100)
            .mode(GameMode::Osu)
            .await?;
        let neme = osu
            .user("Neme")
            .mode(GameMode::Osu)
            .await
            .expect("user not found");
        let neme_scores = osu
            .user_scores(neme.user_id)
            .best()
            .limit(100)
            .mode(GameMode::Osu)
            .await?;
        let mut xeamx_map = HashMap::new();
        let mut neme_map = HashMap::new();
        for score in xeamx_scores {
            xeamx_map.insert(score.map_id, OsuScore::new(score));
        }
        for score in neme_scores {
            neme_map.insert(score.map_id, OsuScore::new(score));
        }
        {
            let mut mutex = data.players_info.lock().unwrap();
            mutex.insert(
                xeamx.username.to_string(),
                PlayerInfo {
                    pp_info: PpInfo {
                        current_pp: xeamx.clone().statistics.expect("not found").pp,
                        top_scores: xeamx_map,
                    },
                    session: SessionInfo::new_with_user(xeamx.clone()),
                },
            );
            mutex.insert(
                neme.username.to_string(),
                PlayerInfo {
                    pp_info: PpInfo {
                        current_pp: neme.clone().statistics.expect("not found").pp,
                        top_scores: neme_map,
                    },
                    session: SessionInfo::new_with_user(neme.clone()),
                },
            );
        }
        info!("osu setup done");
        info!(
            "{} {}",
            xeamx.statistics.expect("not found").pp,
            neme.statistics.expect("not found").pp
        );
        Ok(())
    }

    #[instrument(level = "info", skip_all)]
    async fn update_scores(data: &Arc<OsuData>, current_user: UserExtended, osu: &Osu) -> Score {
        let new_scores = osu
            .user_scores(&current_user.username.to_string())
            .best()
            .limit(100)
            .mode(GameMode::Osu)
            .await
            .expect("");
        let mut new_maps = HashMap::new();
        for score in new_scores.clone() {
            new_maps.insert(score.map_id, OsuScore::new(score));
        }
        info!("Trying to update scores.");
        {
            let mut data = data.players_info.lock().unwrap();
            data.get_mut(&current_user.username.to_string())
                .unwrap()
                .pp_info
                .top_scores = new_maps;
        }
        info!("Done updating scores.");
        new_scores
            .iter()
            .max_by_key(|x| x.ended_at)
            .expect("no score found")
            .clone()
    }

    #[instrument(level = "info", skip_all)]
    async fn update_pp(
        ctx: &poise::serenity_prelude::Context,
        data: &Arc<OsuData>,
        current_user: UserExtended,
        old_pp: f32,
    ) {
        info!("Started update pp sequence.");
        //OsuScore::ember_user(ctx, data, current_user.clone(), dif_pp).await;
        // Creates discord embed and sends it
        let user_embed = CuchaEmbed::new_user_embed(data, current_user.clone(), Some(old_pp));
        let builder = CreateMessage::new().embed(user_embed);
        ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, builder)
            .await
            .expect("could not send message");

        let current_pp = current_user.clone().statistics.unwrap().pp;
        info!("Cloned current pp");
        {
            match data.players_info.lock() {
                Ok(mut data) => match data.get_mut(&current_user.username.to_string()) {
                    Some(player_info) => player_info.pp_info.current_pp = current_pp,
                    None => error!("The name was not found in the data tuple"),
                },
                Err(why) => println!("{}", why),
            }
        }
        info!("Updated data with the new pp score");
    }

    #[instrument(level = "info", skip_all)]
    async fn sends_message(
        ctx: &poise::serenity_prelude::Context,
        data: &Arc<OsuData>,
        score: Score,
    ) {
        let embed_score = CuchaEmbed::new_score_embed(&ctx.http, data, score.clone()).await;
        let builder = CreateMessage::new().embed(embed_score);
        ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, builder)
            .await
            .expect("could not send message");
        if score.is_perfect_combo {
            let perfect_builder =
                CreateMessage::new().embed(CreateEmbed::new().image(PERFECT_IMAGE));
            ChannelId::new(OSU_SPAM_CHANNEL_ID)
                .send_message(&ctx, perfect_builder)
                .await
                .expect("could not send message");
        } else if score.statistics.combo_break == 0 && score.statistics.miss == 0 {
            let fc_builder = CreateMessage::new().embed(CreateEmbed::new().image(FULL_COMBO));
            ChannelId::new(OSU_SPAM_CHANNEL_ID)
                .send_message(&ctx, fc_builder)
                .await
                .expect("could not send message");
        };
    }
}
