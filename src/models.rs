use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OAuth {
    pub access_token: String,
    pub expires: u64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub country: String,
    pub picture_medium: String,
}

#[derive(Deserialize)]
pub struct ListeningHistoryArtist {
    pub name: String,
}

#[derive(Deserialize)]
pub struct ListeningHistoryAlbum {
    #[serde(rename(deserialize = "cover_medium"))]
    pub picture: String,
}

#[derive(Deserialize)]
pub struct ListeningHistoryItem {
    pub id: u64,
    pub title: String,
    pub rank: u64,
    pub artist: ListeningHistoryArtist,
    pub album: ListeningHistoryAlbum,
}

#[derive(Deserialize)]
pub struct ListeningHistory {
    pub data: Vec<ListeningHistoryItem>,
    pub next: Option<String>,
}

#[derive(Deserialize)]
pub struct Playlists {
    pub total: u64,
}

#[derive(Deserialize)]
pub struct LovedTracksArtist {
    pub id: u64,
    pub name: String,
    pub picture_medium: String,
}

#[derive(Deserialize)]
pub struct LovedTracksItem {
    pub duration: u64,
    pub rank: u64,
    #[serde(rename(deserialize = "explicit_lyrics"))]
    pub explicit: bool,
    pub artist: LovedTracksArtist
}

#[derive(Deserialize)]
pub struct LovedTracks {
    pub data: Vec<LovedTracksItem>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeezStatsAverage {
    pub minutes: u64,
    pub seconds: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DeezStatsUniqueness {
    pub recent: u64,
    pub loved: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DeezStatsNumbers {
    pub loved_tracks: u64,
    pub playlists: u64,
    pub recent_tracks: u64,
    pub explicit: u64,
    pub average: DeezStatsAverage,
    pub uniqueness: DeezStatsUniqueness,
}

#[derive(Serialize, Deserialize)]
pub struct DeezStatsTopArtist {
    pub name: String,
    pub picture: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeezStatsTopTrack {
    pub title: String,
    pub name: String,
    pub picture: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeezStats {
    pub success: bool,
    pub date: String,
    pub user: User,
    pub numbers: DeezStatsNumbers,
    pub artists: Vec<DeezStatsTopArtist>,
    pub tracks: Vec<DeezStatsTopTrack>,
}
