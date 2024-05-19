use crate::prelude::*;
use crate::Data;
use rosu_v2::error;
use rosu_v2::prelude::*;



pub struct OsuClient {
    osu: Osu
}

impl OsuClient {
    pub async fn new() -> OsuResult<OsuClient>{
        dotenv().ok();
        let client_id = env::var("OSU_CLIENT_ID").expect("osu id not found").parse::<u64>().expect("error parsing id");
        let client_secret = String::from(env::var("OSU_CLIENT_SECRET").expect("osu secret not found"));
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