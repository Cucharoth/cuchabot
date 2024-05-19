use chatgpt::prelude::Conversation;
use poise::serenity_prelude::User;
use crate::data::bst::*;
use crate::prelude::*;

pub struct Data {
    pub(crate) discord_thread_info: Mutex<HashMap<u64, Conversation>>,
    pub thread_info_as_bst: Mutex<BST<(User, Conversation)>>,
    pub first_message: Mutex<String>,
    pub osu_info: Mutex<(u64, String)>
}