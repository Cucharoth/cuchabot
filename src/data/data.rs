use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use chatgpt::prelude::Conversation;
use poise::serenity_prelude::User;
use crate::data::bst::*;
use crate::osu::dto::dto_osu_score::OsuScore;
use crate::prelude::*;

pub struct Data {
    pub(crate) discord_thread_info: Mutex<HashMap<u64, Conversation>>,
    pub thread_info_as_bst: Mutex<BST<(User, Conversation)>>,
    pub first_message: Mutex<String>,
    pub osu_info: Mutex<(u64, String)>,
    pub osu_pp: Mutex<HashMap<String, (f32, HashMap<u32, OsuScore>)>>,
    pub is_loop_running: AtomicBool,
    pub secret_store: Mutex<SecretStore>,
    pub cuchabot: Arc<Ready> 
}