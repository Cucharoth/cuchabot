use std::sync::Arc;

use crate::osu::dto::dto_osu_score::OsuScore;
use crate::prelude::*;

pub struct OsuData {
    pub osu_pp: Mutex<HashMap<String, (f32, HashMap<u32, OsuScore>)>>,
    pub osu_info: Mutex<(u64, String)>,
    pub cuchabot: Arc<Ready>
}

impl OsuData {
    pub fn new(data: &Data) -> Self{
        let id;
        let client_secret;
        let cuchabot;
        {
            let secrets = data.secret_store.lock().unwrap();
            id = secrets.get("OSU_CLIENT_ID").expect("").parse::<u64>().expect("");
            client_secret = secrets.get("OSU_CLIENT_SECRET").expect("");
        }
        {
            cuchabot = data.cuchabot.clone();
        }
        Self { 
            osu_pp: Mutex::new(HashMap::new()),
            osu_info: Mutex::new((id, client_secret)),
            cuchabot,
        }
    }
}