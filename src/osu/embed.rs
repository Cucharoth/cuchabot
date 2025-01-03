use std::sync::Arc;

use num_format::{Locale, ToFormattedString};
use poise::serenity_prelude::{
    Color, CreateEmbed, CreateEmbedAuthor, EmojiId, GuildId, Http, MessageBuilder,
};
use rosu_v2::prelude::{Score, UserExtended};

use crate::data::osu_data::{OsuData, SessionInfo};

// get the id with '\' before an emoji on discord chat
const EMOJI_INFO: [(&str, u64); 17] = [
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
    ("STAR", 1317609239321251880),
    ("DT", 1317624011781312583),
    ("HD", 1317624422940672181),
    ("HR", 1317624421489315971),
    ("NC", 1317624424924446850),
    ("FL", 1317624419249688717),
    ("PF", 1317624426195320973),
];

const OSU_THUMBNAIL: &str = "https://i.ppy.sh/013ed2c11b34720790e74035d9f49078d5e9aa64/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f4272616e645f6964656e746974795f67756964656c696e65732f696d672f75736167652d66756c6c2d636f6c6f75722e706e67";
pub const EMOJI_GUILD_ID: u64 = 1002656027088523286;

pub struct CuchaEmbed;

impl CuchaEmbed {
    pub fn new_session_embed(osu_data: &Arc<OsuData>, session: &SessionInfo) -> CreateEmbed {
        let author_name = String::from(&osu_data.cuchabot.user.name);
        let img_author = String::from(&osu_data.cuchabot.user.avatar_url().unwrap());
        let author = CreateEmbedAuthor::new(author_name).icon_url(img_author);
        let title = format!("{} session recap", &session.initial_user.username);
        let mut description = format!(
            "lv: {}\n",
            &session
                .updated_user
                .statistics
                .clone()
                .unwrap()
                .level
                .current
        );
        description += &format!(
            "pp: {:.1}\n",
            &session.updated_user.statistics.clone().unwrap().pp
        );
        description += &format!(
            "rank: {}\n",
            &session
                .updated_user
                .statistics
                .clone()
                .unwrap()
                .global_rank
                .unwrap()
                .to_formatted_string(&Locale::en)
        );
        let thumbnail = &session.updated_user.avatar_url;
        let color = Color::MAGENTA;
        let pp_gain = format!("+{:.1}", session.get_pp_gain());
        let rank_gain = format!(
            "+{}",
            session.get_rank_gain().to_formatted_string(&Locale::en)
        );
        let mut fields = vec![];
        fields.push(("PP gain", pp_gain, true));
        fields.push(("Rank gain", rank_gain, true));
        CreateEmbed::new()
            .author(author)
            .title(title)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            .fields(fields)
    }

    pub fn new_user_embed(
        data: &Arc<OsuData>,
        current_user: UserExtended,
        old_pp: Option<f32>,
    ) -> CreateEmbed {
        let author_name = String::from(&data.cuchabot.user.name);
        let img_author = String::from(&data.cuchabot.user.avatar_url().unwrap());
        let description = format!("{} got a new score!", current_user.clone().username);
        let title = current_user.username.clone().to_string();
        let thumbnail = &current_user.clone().avatar_url;
        let color = Color::DARK_RED;
        let current_pp = current_user.clone().statistics.unwrap().pp;
        let mut pp_diff = 0.;
        let pp_field = format!(
            "{old_pp:.1} -> {current_pp:.1} (+{diff:.1})",
            old_pp = if let Some(old_pp) = old_pp {
                pp_diff = current_pp - old_pp;
                old_pp
            } else {
                0.
            },
            current_pp = current_pp,
            diff = pp_diff
        );
        let author = CreateEmbedAuthor::new(author_name).icon_url(img_author);
        CreateEmbed::new()
            .author(author)
            .title(title)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            .field("Current PP", pp_field, true)
    }

    pub async fn new_score_embed(http: &Http, data: &Arc<OsuData>, score: Score) -> CreateEmbed {
        let title = score.mapset.clone().unwrap().title;
        let author_name = String::from(&data.cuchabot.user.name);
        let mut description = format!("by: {} \n", score.mapset.clone().unwrap().artist);
        description += &format!(
            "mapped by: {} \n",
            score.mapset.clone().unwrap().creator_name
        );
        description += &format!("'{}'", score.map.clone().unwrap().version);
        let img_author = String::from(&data.cuchabot.user.avatar_url().unwrap());
        let author = CreateEmbedAuthor::new(author_name).icon_url(img_author);
        let thumbnail = OSU_THUMBNAIL;
        let color = Color::FABLED_PINK;
        let accuracy = format!("{acc:.1}%", acc = score.accuracy);
        let grade = Self::get_any_emoji(http, score.grade.to_string()).await;
        let star_emoji = Self::get_any_emoji(http, "STAR".to_string()).await;
        let stars = format!("{}{}", score.map.clone().unwrap().stars, star_emoji);
        let image = score.clone().mapset.unwrap().covers.cover;
        let pp = format!(
            "{pp:.1}",
            pp = if let Some(pp) = score.pp { pp } else { 0. }
        );
        let max_combo = score.max_combo.to_string();
        let map_score = score.legacy_score.to_formatted_string(&Locale::en);
        let source = score.map.clone().unwrap().url;
        let weight = format!(
            "{score:.1}%",
            score = if let Some(weight) = score.weight {
                weight.percentage
            } else {
                0.
            }
        );
        let mut fields = vec![];
        fields.push(("Grade", grade, true));
        fields.push(("Accuracy", accuracy, true));
        fields.push(("PP", pp.to_string(), true));
        fields.push(("Max Combo", max_combo, true));
        fields.push(("Score", map_score, true));
        fields.push(("Weight", weight, true));
        if score.mods.len() > 1 {
            let mods = Self::add_mods(http, &score).await;
            fields.push(("Mods", mods, false));
        }
        CreateEmbed::new()
            .title(title)
            .author(author)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            .image(image)
            .field("Stars", stars, false)
            .fields(fields)
            .url(source)
    }

    pub async fn add_mods(http: &Http, score: &Score) -> String {
        let mut mods_emojis = String::new();
        for mod_item in &score.mods {
            // ignore the 'Classic osu' mod
            if !mod_item.acronym().as_str().contains("CL") {
                mods_emojis += &Self::get_any_emoji(http, mod_item.acronym().to_string()).await;
                mods_emojis += "\t\t\t";
            }
        }
        mods_emojis
    }

    async fn get_any_emoji(http: &Http, name: String) -> String {
        if let Some(emoji_id) = EMOJI_INFO.iter().find(|x| x.0 == &name).map(|x| x.1) {
            match http
                .get_emoji(GuildId::from(EMOJI_GUILD_ID), EmojiId::from(emoji_id))
                .await
            {
                Ok(emoji) => MessageBuilder::new().emoji(&emoji).build(),
                Err(why) => {
                    println!("Failed to fetch emoji '{}': {}", name, why);
                    "".to_string()
                }
            }
        } else {
            return format!(
                "\nCucha no ha implementado el mod {} yet\n ¯\\_(ツ)_/¯ \n",
                name
            );
        }
    }
}
