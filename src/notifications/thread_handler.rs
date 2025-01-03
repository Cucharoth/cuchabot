use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

use rosu_v2::{error::OsuError, model::GameMode};
use serenity_prelude::ActivityData;

use crate::{
    osu::{get_users, osu_client::OsuClient, pp_check::PpCheck, user_activity::UserActivity},
    prelude::*,
};

use tracing::{error, info, instrument};

pub struct ThreadHandler;

impl ThreadHandler {
    #[instrument(level = "info", skip_all)]
    pub async fn new(
        old_ctx: &poise::serenity_prelude::Context,
        old_data: Arc<Db>,
    ) -> Result<(), OsuError> {
        let ctx_arc = Arc::new(old_ctx.clone());
        PpCheck::setup(old_ctx, &old_data.osu_data).await?;
        if !&old_data.is_loop_running.load(Ordering::Relaxed) {
            old_data.is_loop_running.store(true, Ordering::Relaxed);
            let ctx = Arc::clone(&ctx_arc);
            let data = Arc::clone(&old_data);
            tokio::spawn(async move {
                loop {
                    match OsuClient::new_from_thread(&ctx, &data.osu_data).await {
                        Ok(osu_client) => {
                            let osu = osu_client.osu;
                            let users = get_users(&data.osu_data);
                            for current_username in users {
                                let current_user = osu
                                    .user(current_username.clone())
                                    .mode(GameMode::Osu)
                                    .await
                                    .expect("could not load user");

                                // handles user activity checks
                                if let Err(why) = UserActivity::user_activity(
                                    &ctx,
                                    &data,
                                    &osu,
                                    &current_user,
                                )
                                .await
                                {
                                    error!("user activity error: {}", why);
                                }
                            }

                            let currently_playing = data.osu_data.get_currently_playing();
                            // resets bot activity
                            if currently_playing == 0 && data.osu_data.watching_gameplay.load(Ordering::Relaxed){
                                info!("activity reset");
                                ctx.set_activity(Some(ActivityData::playing("Osu!")));
                                data.osu_data.watching_gameplay.store(false, Ordering::Relaxed);
                            }
                        }
                        Err(why) => error!("error getting an osu client: {}", why),
                    }

                    tokio::time::sleep(Duration::from_secs(120)).await;
                }
            });
        }
        Ok(())
    }
}
