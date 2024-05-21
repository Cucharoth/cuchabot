use rosu_v2::prelude::*;

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