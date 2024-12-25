use std::sync::Arc;

use crate::data::osu_data::OsuData;

pub mod osu_client;
pub mod dto;
pub mod pp_check;
pub mod user_activity;

pub fn get_users(data: &Arc<OsuData>) -> Vec<String> {
    let data_mutex = data.players_info.lock().unwrap();
    data_mutex.keys().cloned().collect()
}