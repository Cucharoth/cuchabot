use crate::prelude::*;
use crate::Data;
use rosu_v2::error;
use rosu_v2::prelude::*;
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct OsuClient {
    osu: Osu
}

impl OsuClient {
    pub async fn new(ctx: Context<'_>) -> OsuResult<OsuClient>{
        let client_id = { ctx.data().osu_info.lock().unwrap().0 };
        let client_secret = { String::from(&ctx.data().osu_info.lock().unwrap().1) };
        Ok(Self {
            osu : Osu::new(client_id, client_secret).await?
        })
    }

    pub async fn getBestScores(&self, user_name: &str) -> Vec<Score> {
        self.osu.user_scores(user_name)
            .mode(GameMode::Osu)
            .best()
            .limit(1)
            .await
            .unwrap_or_else(|why| panic!("Failed to get scores: {}", why))
    }

    pub async fn getRecentScores(&self, user_name: &str) -> Vec<Score> {
        self.osu.user_scores(user_name)
            .mode(GameMode::Osu)
            .recent()
            .limit(1)
            .await
            .unwrap_or_else(|why| panic!("Failed to get scores: {}", why))
    }
}