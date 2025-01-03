use poise::serenity_prelude::{ChannelId, Context, CreateEmbed, CreateMessage};
use tracing::error;

pub const OSU_SPAM_CHANNEL_ID: u64 = 1242292133965205597;
pub const PERFECT_IMAGE: &str = "https://i.imgur.com/rhmsFV2.png";
pub const FULL_COMBO: &str = "https://i.imgur.com/sHJRcxA.png";

pub struct CuchabotMessage {}

impl CuchabotMessage {
    pub async fn new_msg_spam_channel(ctx: &Context, content: String) {
        let builder = CreateMessage::new().content(content);
        let response = ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, builder)
            .await;
        if let Err(why) = response {
            error!("{}", why);
        }
    }

    pub async fn new_msg_spam_channel_with_embed(ctx: &Context, embed: CreateEmbed) {
        let builder = CreateMessage::new().embed(embed);
        let response = ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, builder)
            .await;
        if let Err(why) = response {
            error!("{}", why);
        }
    }

    pub async fn spam_channel_osu_perfect(ctx: &Context) {
        let perfect_builder = CreateMessage::new().embed(CreateEmbed::new().image(PERFECT_IMAGE));
        let response = ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, perfect_builder)
            .await;
        if let Err(why) = response {
            error!("{}", why);
        }
    }

    pub async fn spam_channel_osu_fc(ctx: &Context) {
        let fc_builder = CreateMessage::new().embed(CreateEmbed::new().image(FULL_COMBO));
        let response = ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, fc_builder)
            .await;
        if let Err(why) = response {
            error!("{}", why);
        }
    }

    pub async fn new_msg_spam_channel_with_builder(ctx: &Context, builder: CreateMessage) {
        let response = ChannelId::new(OSU_SPAM_CHANNEL_ID)
            .send_message(&ctx, builder)
            .await;
        if let Err(why) = response {
            error!("{}", why);
        }
    }
}
