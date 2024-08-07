use std::{collections::HashMap, sync::Arc};
use num_format::{Locale, ToFormattedString};

use poise::{serenity_prelude::{model::colour, ChannelId, Color, CreateEmbed, CreateEmbedAuthor, CreateMessage, EmbedThumbnail, EmojiId, Guild, MessageBuilder}, CreateReply};
use rosu_v2::prelude::*;

use crate::{data::osu_data::OsuData, Data, EMOJI_GUILD_ID, OSU_SPAM_CHANNEL_ID};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// get the id with '\' before an emoji on discord chat
const EMOJI_INFO: [(&str, u64); 10] = [
    ("XH", 1246392671057219584),
    ("X", 1246392672919355444),
    ("SH", 1246392667974271008),
    ("S", 1246392669245276171),
    ("D", 1246392666506264606),
    ("C", 1246392664887394304),
    ("B", 1246392663234580542),
    ("A", 1246392661418577930),
    ("PERFECT", 1265878344998846614),
    ("FULL_COMBO", 1265877040419049502),
];

const PERFECT_IMAGE: &str = "https://i.imgur.com/rhmsFV2.png";
const FULL_COMBO: &str = "https://i.imgur.com/sHJRcxA.png";

#[derive(Debug)]
pub struct OsuScore {
    pub ranked: Option<bool>,
    pub grade: Grade,
    pub accuracy: f32,
    pub is_perfect_combo: bool,
    pub max_combo: u32,
    pub passed: bool,
    pub pp: Option<f32>,
    pub score: u32,
    pub mapset: Option<BeatMap>
}

impl OsuScore {
    pub fn new(score: Score) -> Self {
        Self {
            ranked: score.ranked,
            grade: score.grade,
            accuracy: score.accuracy,
            is_perfect_combo: score.is_perfect_combo,
            max_combo: score.max_combo,
            passed: score.passed,
            pp: score.pp,
            score: score.score,
            mapset: if let Some(mapset) = score.mapset {
                BeatMap::new(mapset)
            } else {None}
        }
    }

    pub async fn ember_user_from_command(ctx: Context<'_>, data: &Arc<OsuData>, current_user: UserExtended) {
        let author_name = String::from(&data.cuchabot.user.name);
        let img_author = String::from(&data.cuchabot.user.avatar_url().unwrap());
        let description = format!("{} got a new score!", current_user.clone().username);
        let title = current_user.username.clone().to_string();
        let thumbnail = &current_user.clone().avatar_url;
        let color = Color::DARK_RED;
        let img = &current_user.cover.url;
        let pp = current_user.clone().statistics.unwrap().pp.to_string();
        let author = CreateEmbedAuthor::new(author_name).icon_url(img_author);
        let embed = CreateEmbed::new()
            .author(author)
            .title(title)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            .image(img)
            .field("Current PP", pp, true);
        let builder = CreateReply::default().embed(embed);
        ctx.send(builder).await.expect("could not send the message.");
    }

    pub async fn ember_user(ctx: &poise::serenity_prelude::Context, data: &Arc<OsuData>, current_user: UserExtended, dif_pp: f32) {
        let url = current_user.page.clone().unwrap().html;
        let author_name = String::from(&data.cuchabot.user.name);
        let img_author = String::from(&data.cuchabot.user.avatar_url().unwrap());
        let description = format!("{} got a new score!", current_user.clone().username);
        let title = current_user.username.clone().to_string();
        let thumbnail = &current_user.clone().avatar_url;
        let color = Color::DARK_RED;
        let img = &current_user.cover.url;
        let pp = current_user.clone().statistics.unwrap().pp.to_string();
        let author = CreateEmbedAuthor::new(author_name).icon_url(img_author);
        let embed = CreateEmbed::new()
            .author(author)
            .title(title)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            //.image(img)
            .field("Current PP", pp, true)
            .field(" ", format!("+{}", format!("{pp:.1}", pp = dif_pp)), true)
            .field(" ", " ", true)
            .url(url);
        let builder = CreateMessage::new().embed(embed);
        ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, builder).await.expect("could not send message");
    }

    pub async fn embed_ranked_score_from_command(ctx: Context<'_>, data: &Arc<OsuData>, score: Score) {
        let title = score.mapset.clone().unwrap().title;
        let author_name = String::from(&data.cuchabot.user.name);
        let mut description = format!("by: {} \n", score.mapset.clone().unwrap().artist);
        description += &format!("mapped by: {} \n", score.mapset.clone().unwrap().creator_name);
        description += &format!("'{}'", score.map.clone().unwrap().version);
        let img_author = String::from(&data.cuchabot.user.avatar_url().unwrap());
        let author = CreateEmbedAuthor::new(author_name).icon_url(img_author);
        let thumbnail = "https://i.ppy.sh/013ed2c11b34720790e74035d9f49078d5e9aa64/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f4272616e645f6964656e746974795f67756964656c696e65732f696d672f75736167652d66756c6c2d636f6c6f75722e706e67";
        let color = Color::FABLED_PINK;
        let accuracy = format!("{acc:.1}", acc = score.accuracy);
        let grade = Self::get_emoji_from_cmd(ctx, score.grade.to_string());
        let stars = score.map.clone().unwrap().stars.to_string();
        let image = score.mapset.unwrap().covers.cover;
        let pp = format!("{pp:.1}", pp = if let Some(pp) = score.pp {pp} else {0.});
        let max_combo = score.max_combo.to_string();
        let map_score = score.legacy_score.to_formatted_string(&Locale::en);
        let source = score.map.clone().unwrap().url;
        let weight = format!("{score:.1}%", score = if let Some(weight) = score.weight {weight.percentage} else {0.});
        let mut fields = vec![];
        fields.push(("Grade", grade, true));
        fields.push(("Accuracy", accuracy, true));
        fields.push(("PP", pp.to_string(), true));
        fields.push(("Max Combo", max_combo, true));
        fields.push(("Score", map_score, true));
        fields.push(("Weight", weight, true));
        let embed = CreateEmbed::new()
            .title(title)
            .author(author)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            .image(image)
            .field("Stars", stars, false)
            .fields(fields)
            .url(source);
        let builder = CreateReply::default().embed(embed);
        ctx.send(builder).await.expect("could not send the message.");
        if score.is_perfect_combo {
            let perfect_builder = CreateReply::default().embed(CreateEmbed::new().image(PERFECT_IMAGE));
            ctx.send(perfect_builder).await.expect("could not send message");
        } else if score.statistics.combo_break == 0 && score.statistics.miss == 0 {
            let fc_builder = CreateReply::default().embed(CreateEmbed::new().image(FULL_COMBO));
            ctx.send(fc_builder).await.expect("could not send message");
        };
    }

    pub async fn embed_ranked_score(ctx: &poise::serenity_prelude::Context, score: Score, data: &Arc<OsuData>) {
        let title = score.mapset.clone().unwrap().title;
        let author_name = String::from(&data.cuchabot.user.name);
        let mut description = format!("by: {} \n", score.mapset.clone().unwrap().artist);
        description += &format!("mapped by: {} \n", score.mapset.clone().unwrap().creator_name);
        description += &format!("'{}'", score.map.clone().unwrap().version);
        let img_author = String::from(&data.cuchabot.user.avatar_url().unwrap());
        let author = CreateEmbedAuthor::new(author_name).icon_url(img_author);
        let thumbnail = "https://i.ppy.sh/013ed2c11b34720790e74035d9f49078d5e9aa64/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f4272616e645f6964656e746974795f67756964656c696e65732f696d672f75736167652d66756c6c2d636f6c6f75722e706e67";
        let color = Color::FABLED_PINK;
        let accuracy = format!("{acc:.1}", acc = score.accuracy);
        let grade = Self::get_emoji(ctx, score.grade.to_string());
        let stars = score.map.clone().unwrap().stars.to_string();
        let image = score.mapset.unwrap().covers.cover;
        let pp = format!("{pp:.1}", pp = if let Some(pp) = score.pp {pp} else {0.});
        let max_combo = score.max_combo.to_string();
        let map_score = score.legacy_score.to_formatted_string(&Locale::en);
        let source = score.map.clone().unwrap().url;
        let weight = format!("{score:.1}%", score = if let Some(weight) = score.weight {weight.percentage} else {0.});
        let mut fields = vec![];
        fields.push(("Grade", grade, true));
        fields.push(("Accuracy", accuracy, true));
        fields.push(("PP", pp, true));
        fields.push(("Max Combo", max_combo, true));
        fields.push(("Score", map_score, true));
        fields.push(("Weight", weight, true));
        let embed = CreateEmbed::new()
            .title(title)
            .author(author)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            .image(image)
            .field("Stars", stars, false)
            .fields(fields)
            .url(source);
        let builder = CreateMessage::new().embed(embed);
        ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, builder).await.expect("could not send message");
        if score.is_perfect_combo {
            let perfect_builder = CreateMessage::new().embed(CreateEmbed::new().image(PERFECT_IMAGE));
            ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, perfect_builder).await.expect("could not send message");
        } else if score.statistics.combo_break == 0 && score.statistics.miss == 0 {
            let fc_builder = CreateMessage::new().embed(CreateEmbed::new().image(FULL_COMBO));
            ChannelId::new(OSU_SPAM_CHANNEL_ID).send_message(&ctx, fc_builder).await.expect("could not send message");
        };
    }

    fn get_emoji_from_cmd(ctx: Context<'_>, name: String) -> String {
        let emoji_id = EMOJI_INFO.iter().find(|x| x.0 == &name).unwrap().1;
        {
            let guild = ctx.cache().guild(EMOJI_GUILD_ID).unwrap();
            let emoji = guild.emojis.get(&EmojiId::new(emoji_id)).unwrap();
            MessageBuilder::new().emoji(emoji).build()
        }
    }

    fn get_emoji(ctx: &poise::serenity_prelude::Context, name: String) -> String {
        let emoji_id = EMOJI_INFO.iter().find(|x| x.0 == &name).unwrap().1;
        {
            let guild = ctx.cache.guild(EMOJI_GUILD_ID).unwrap();
            let emoji = guild.emojis.get(&EmojiId::new(emoji_id)).unwrap();
            MessageBuilder::new().emoji(emoji).build()
        }
    }
}

#[derive(Debug)]
pub struct BeatMap {
    pub title: String,
    pub artist: String,
    pub creator_name: Username,
    pub preview_url: String,
    pub source: String,
    pub cover: String,
}

impl BeatMap {
    pub fn new(beat_map_set: Box<Beatmapset>) -> Option<Self> {
        Some(Self {
            title: beat_map_set.title,
            artist: beat_map_set.artist,
            creator_name: beat_map_set.creator_name,
            preview_url: beat_map_set.preview_url,
            source: beat_map_set.source,
            cover: beat_map_set.covers.cover
        })
    }
}