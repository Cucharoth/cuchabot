use std::{sync::{atomic::{Ordering}, Arc}, time::Duration};

use poise::serenity_prelude::{ChannelId, CreateMessage};
use rosu_v2::error::OsuError;

use crate::{data::osu_data::OsuData, osu::pp_check::PpCheck, prelude::*};

pub struct ThreadHandler;

impl ThreadHandler {
    pub async fn new(ctx: &poise::serenity_prelude::Context, osu_data: Arc<OsuData>, data: &Data) -> Result<(), OsuError>{
        let ctx_arc = Arc::new(ctx.clone());
        PpCheck::setup(ctx, &osu_data).await?;
        if !&data.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx_arc);
            let osu_data1 = osu_data.clone();
            tokio::spawn(async move {
                loop {
                    //let builder = CreateMessage::new().content("this is a loop e_e");
                    //ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx1, builder).await.expect("wrong channel id");
                    PpCheck::check_current_pp(&ctx1, &osu_data1).await.expect("Error loading user");
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });
        }
        Ok(())
    }
}