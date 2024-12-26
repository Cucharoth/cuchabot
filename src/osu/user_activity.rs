use std::sync::{atomic::Ordering, Arc};

use poise::serenity_prelude::{
    ActivityData, ChannelId, Context, CreateEmbed, CreateMessage, MessageFlags
};
use rosu_v2::{error::OsuError, prelude::UserExtended, Osu};
use tracing::{error, info, instrument};

use crate::{
    data::osu_data::OsuData,
    osu::{embed::CuchaEmbed, pp_check::PpCheck},
    OSU_SPAM_CHANNEL_ID,
};

pub struct UserActivity;

impl UserActivity {
    #[instrument(level = "info", skip_all)]
    pub async fn user_activity(
        ctx: &Context,
        osu_data: &Arc<OsuData>,
        osu: &Osu,
        current_user: &UserExtended,
    ) -> Result<(), OsuError> {
        let username = current_user.username.to_string();
        let should_report = OsuData::get_should_report(osu_data, &username);
        if current_user.is_online {
            info!("{} is online", username);

            osu_data.set_is_playing(&username);

            if !should_report {
                osu_data.set_should_report(&username);
                osu_data.init_session(&username, current_user);
            }
            let currently_playing = osu_data.get_currently_playing();

            Self::edit_bot_activity(ctx, osu_data, username, currently_playing);

            // handles scoring checking + embedding message on dc
            if let Err(why) = PpCheck::check_current_pp(&ctx, &osu_data, &osu, &current_user).await
            {
                error!("check current pp error: {}", why);
            };
        } else {
            info!("{} is offline", username);
            if should_report {
                Self::make_report(ctx, osu_data, &username).await;
            }
        }

        Ok(())
    }

    #[instrument(level = "info", skip_all)]
    pub fn update_session(
        _ctx: &poise::serenity_prelude::Context,
        osu_data: &Arc<OsuData>,
        updated_user: &UserExtended,
    ) {
        info!("updating session");
        let current_username = &updated_user.username.to_string();
        {
            let mut data_mutex = osu_data.players_info.lock().unwrap();
            let player_info = data_mutex.get_mut(current_username).unwrap();
            player_info.session.updated_user = updated_user.clone();
            player_info.session.improvement_counter += 1;
        }
    }

    #[instrument(level = "info", skip_all)]
    async fn make_report(ctx: &Context, osu_data: &Arc<OsuData>, username: &str) {
        info!("making report for: {}", username);
        let session = {
            let mutex = osu_data.players_info.lock().unwrap();
            mutex.get(username).unwrap().session.clone()
        };
        let report_embed = CuchaEmbed::new_session_embed(osu_data, &session);
        Self::send_report(ctx, report_embed).await;
        osu_data.reset_session(username);
    }

    async fn send_report(ctx: &Context, embed: CreateEmbed) {
        let builder = CreateMessage::new()
            .embed(embed)
            .flags(MessageFlags::SUPPRESS_NOTIFICATIONS);
        ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, builder)
            .await
            .expect("could not send message");
    }

    fn edit_bot_activity(
        ctx: &poise::serenity_prelude::Context,
        data: &Arc<OsuData>,
        username: String,
        currently_playing: usize,
    ) {
        if currently_playing == 1 {
            let status = username;
            ctx.set_activity(Some(ActivityData::watching(status)));
            data.watching_gameplay.store(true, Ordering::Relaxed);
        }
    }
}
