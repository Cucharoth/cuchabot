use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use rosu_v2::prelude::UserExtended;
use tracing::error;

use crate::osu::dto::dto_osu_score::OsuScore;
use crate::prelude::*;

pub struct OsuData {
    //pub osu_pp: Mutex<HashMap<String, (f32, HashMap<u32, OsuScore>)>>,
    pub players_info: Mutex<HashMap<String, PlayerInfo>>,
    pub osu_info: Mutex<(u64, String)>,
    pub cuchabot: Arc<Ready>,
    pub watching_gameplay: AtomicBool,
}

impl OsuData {
    pub fn new(data: &Data) -> Self {
        let id;
        let client_secret;
        let cuchabot;
        {
            let secrets = data.secret_store.lock().unwrap();
            id = secrets
                .get("OSU_CLIENT_ID")
                .expect("")
                .parse::<u64>()
                .expect("");
            client_secret = secrets.get("OSU_CLIENT_SECRET").expect("");
        }
        {
            cuchabot = data.cuchabot.clone();
        }
        Self {
            players_info: Mutex::new(HashMap::new()),
            osu_info: Mutex::new((id, client_secret)),
            cuchabot,
            watching_gameplay: AtomicBool::new(false),
        }
    }

    pub fn get_currently_playing(&self) -> usize {
        let players_info = self.players_info.lock().unwrap();
        players_info
            .values()
            .filter(|player_info| player_info.session.is_playing)
            .count()
    }

    pub fn set_is_playing(&self, username: &str) {
        let mut mutex = self.players_info.lock().unwrap();
        if let Some(player_info) = mutex.get_mut(username) {
            player_info.session.is_playing = true;
        } else {
            error!("User '{}' not found in players_info", username);
        }
    }

    pub fn reset_session(&self, username: &str) {
        let mut mutex = self.players_info.lock().unwrap();
        if let Some(player_info) = mutex.get_mut(username) {
            player_info.session.is_playing = false;
            player_info.session.should_report = false;
        } else {
            error!("User '{}' not found in players_info", username);
        };
    }

    pub fn set_should_report(&self, username: &str) {
        let mut mutex = self.players_info.lock().unwrap();
        if let Some(player_info) = mutex.get_mut(username) {
            player_info.session.should_report = true;
        } else {
            error!("User '{}' not found in players_info", username);
        };
    }

    pub fn get_should_report(&self, username: &str) -> bool {
        let mut mutex = self.players_info.lock().unwrap();
        if let Some(player_info) = mutex.get_mut(username) {
            player_info.session.should_report
        } else {
            error!("User '{}' not found in players_info", username);
            false
        }
    }

    pub fn init_session(&self, username: &str, user: &UserExtended) {
        let mut mutex = self.players_info.lock().unwrap();
        if let Some(player_info) = mutex.get_mut(username) {
            player_info.session.initial_user = user.clone();
            player_info.session.updated_user = user.clone();
        } else {
            error!("User '{}' not found in players_info", username);
        }
    }
}

pub struct PlayerInfo {
    pub pp_info: PpInfo,
    pub session: SessionInfo,
}

pub struct PpInfo {
    pub current_pp: f32,
    pub top_scores: HashMap<u32, OsuScore>,
}
#[derive(Clone)]
pub struct SessionInfo {
    pub is_playing: bool,
    pub initial_user: UserExtended,
    pub should_report: bool,
    pub updated_user: UserExtended,
    pub improvement_counter: usize,
}

impl SessionInfo {
    pub fn new_with_user(user: UserExtended) -> Self {
        Self {
            is_playing: false,
            initial_user: user.clone(),
            should_report: false,
            updated_user: user,
            improvement_counter: 0,
        }
    }

    pub fn get_pp_gain(&self) -> f32 {
        let updated_pp = self.updated_user.statistics.clone().unwrap().pp;
        updated_pp - self.initial_user.statistics.clone().unwrap().pp
    }

    pub fn get_rank_gain(&self) -> u32 {
        let initial_rank = self.initial_user.statistics.clone().unwrap().global_rank.unwrap();
        let updated_rank = self.updated_user.statistics.clone().unwrap().global_rank.unwrap();
        updated_rank - initial_rank
    }

    pub fn get(player_info: &mut PlayerInfo, updated_user: &UserExtended) {}
}
