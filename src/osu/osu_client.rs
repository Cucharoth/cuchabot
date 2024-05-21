use std::sync::Arc;

use crate::data;
use crate::data::osu_data::OsuData;
use crate::prelude::*;
use crate::Data;
use rosu_v2::error;
use rosu_v2::prelude::*;
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct OsuClient {
    pub osu: Osu
}

impl OsuClient {
    pub async fn new(ctx: Context<'_>) -> OsuResult<OsuClient>{
        let client_id = { ctx.data().osu_info.lock().unwrap().0 };
        let client_secret = { String::from(&ctx.data().osu_info.lock().unwrap().1) };
        Ok(Self {
            osu : Osu::new(client_id, client_secret).await?
        })
    }

    pub async fn new_from_thread(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>) -> OsuResult<OsuClient>{
        let client_id = { data.osu_info.lock().unwrap().0 };
        let client_secret = { String::from(&data.osu_info.lock().unwrap().1) };
        Ok(Self {
            osu : Osu::new(client_id, client_secret).await?
        })
    }

    pub async fn getBestScores(&self, user_name: &str) -> Result<Vec<Score>, OsuError> {
        self.osu.user_scores(user_name)
            .mode(GameMode::Osu)
            .best()
            .limit(1)
            .await
    }

    pub async fn getRecentScores(&self, user_name: &str) -> Result<Vec<Score>, OsuError> {
        self.osu.user_scores(user_name)
            .mode(GameMode::Osu)
            .recent()
            .limit(1)
            .await
    }
}