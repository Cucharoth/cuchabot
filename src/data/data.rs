use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::osu::dto::dto_osu_score::OsuScore;
use crate::prelude::*;

use super::osu_data::OsuData;

pub struct Data {
    pub first_message: Mutex<String>,
    pub osu_info: Mutex<(u64, String)>,
    pub osu_pp: Mutex<HashMap<String, (f32, HashMap<u32, OsuScore>)>>,
    pub is_loop_running: AtomicBool,
    pub secret_store: Mutex<SecretStore>,
    pub cuchabot: Arc<Ready>,
    pub osu_data: Arc<OsuData>
}