use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;
use url::Url;
use worker::{event, kv, Request, Response, RouteContext, Router};

mod models;
mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: worker::Env, _ctx: worker::Context) -> Result<Response, worker::Error> {

    utils::log_request(&req);
    utils::set_panic_hook();

    let router: Router<()> = Router::new();

    router
        .get_async("/count", |_req: Request, ctx: RouteContext<()>| async move {
            // Return the current login counter
            let kv: kv::KvStore = ctx.kv("DEEZ_STATS")?;
            if let Ok(Some(login_count_string)) = kv.get("_count").text().await {
                if let Ok(login_count) = login_count_string.parse::<u32>() {
                    return Response::from_json(&json!({"count": login_count}))
                }
            }
            return Response::from_json(&json!({"count": 0}))
        })
        .get_async("/login", |_req: Request, ctx: RouteContext<()>| async move {
            // Initialize the OAuth flow
            return Response::redirect_with_status(Url::parse(&format!(
                "https://connect.deezer.com/oauth/auth.php?app_id={}&redirect_uri=https://deezstats.com/auth&perms=basic_access,listening_history",
                ctx.var("APP_ID")?.to_string()
            ))?, 307);
        })
        .get_async("/auth", |req: Request, ctx: RouteContext<()>| async move {
            // Extract request URL and GET parameters
            for (name, value) in req.url()?.query_pairs().by_ref() {
                // If code has been passed, attempt to retrieve access key from Deezer API
                if name == "code" {
                    if let Ok(body) = reqwest::get(format!(
                        "https://connect.deezer.com/oauth/access_token.php?output=json&app_id={}&secret={}&code={}",
                        ctx.var("APP_ID")?.to_string(),
                        ctx.secret("APP_SECRET")?.to_string(),
                        value
                    )).await {
                        if let Ok(oauth) = body.json::<models::OAuth>().await {
                            // Generate UUID for the session
                            let session = Uuid::new_v4().to_string();
                            // Store token into KV store
                            let kv: kv::KvStore = ctx.kv("DEEZ_STATS")?;
                            kv.put(&session, oauth.access_token)?.expiration_ttl(oauth.expires).execute().await?;
                            // Redirect to display page                             
                            return Response::redirect_with_status(Url::parse(&format!("https://deezstats.com/view?{}", session.to_string()))?, 307);
                        };
                    };
                }
            }
            // If something fails, redirect to the home page
            return Response::redirect_with_status(Url::parse("https://deezstats.com")?, 307);
        })
        .get_async("/generate/:uuid", |_req: Request, ctx: RouteContext<()>| async move {
            if let Some(uuid) = ctx.param("uuid") {
                // Check if session UUID is valid
                let kv: kv::KvStore = ctx.kv("DEEZ_STATS")?;
                if let Ok(Some(token)) = kv.get(uuid).text().await {
                    // If value is not a Deez Stats object
                    if token.chars().nth(0).unwrap_or('{') != '{' {
                        // Prevent multiple generate operations from being run
                        kv.delete(uuid).await?;
                        // Fetch user profile from Deezer API
                        if let Ok(user_response) = reqwest::get(format!("https://api.deezer.com/user/me?access_token={}", token)).await {
                            if let Ok(user_json) = user_response.json::<models::User>().await {
                                // Gather user's listening history from Deezer API
                                let mut history: Vec<models::ListeningHistoryItem> = Vec::new();
                                let mut history_index: u64 = 0;
                                loop {
                                    if let Ok(history_response) = reqwest::get(format!(
                                        "https://api.deezer.com/user/{}/history?access_token={}&index={}",
                                        user_json.id,
                                        token,
                                        history_index
                                    )).await {
                                        if let Ok(history_json) = history_response.json::<models::ListeningHistory>().await {
                                            history_index += history_json.data.len() as u64;
                                            history.extend(history_json.data);
                                            if history_json.next.is_some() {
                                                continue;
                                            }
                                        }
                                    }
                                    break;
                                }
                                // Counters
                                let mut recent_rank: u64 = 0;
                                let mut recent_track_count: HashMap<u64, u64> = HashMap::new();
                                let mut recent_track_data: HashMap<u64, models::DeezStatsTopTrack> = HashMap::new();
                                // Loop over user's recent tracks
                                for recent_track in history {
                                    recent_rank += recent_track.rank;
                                    // Check whether track has been seen before
                                    *recent_track_count.entry(recent_track.id).or_insert(0) += 1;
                                    // Store track data
                                    recent_track_data.insert(recent_track.id, models::DeezStatsTopTrack{
                                        title: recent_track.title,
                                        name: recent_track.artist.name,
                                        picture: recent_track.album.picture,
                                    });
                                }
                                // Sort and extract the top five most recent tracks
                                let mut recent_track_vector: Vec<(&u64, &u64)> = recent_track_count.iter().collect();
                                recent_track_vector.sort_by(|a, b| b.1.cmp(a.1));
                                let mut top_tracks: Vec<models::DeezStatsTopTrack> = Vec::new();
                                for (index, (key, _)) in recent_track_vector.into_iter().enumerate() {
                                    if index == 5 {
                                        break;
                                    }
                                    if let Some(track_data) = recent_track_data.remove(key) {
                                        top_tracks.push(track_data);
                                    };
                                }
                                // Gather number of playlists from Deezer API
                                let mut playlist_count: u64 = 0;
                                if let Ok(playlist_response) = reqwest::get(format!(
                                    "https://api.deezer.com/user/{}/playlists?access_token={}&index=0",
                                    user_json.id,
                                    token
                                )).await {
                                    if let Ok(playlist_json) = playlist_response.json::<models::Playlists>().await {
                                        playlist_count = playlist_json.total
                                    }
                                }
                                // Gather user's loved tracks from Deezer API
                                let mut loved: Vec<models::LovedTracksItem> = Vec::new();
                                let mut loved_index: u64 = 0;
                                loop {
                                    if let Ok(loved_response) = reqwest::get(format!(
                                        "https://api.deezer.com/user/{}/tracks?access_token={}&index={}",
                                        user_json.id,
                                        token,
                                        loved_index
                                    )).await {
                                        if let Ok(loved_json) = loved_response.json::<models::LovedTracks>().await {
                                            loved_index += loved_json.data.len() as u64;
                                            loved.extend(loved_json.data);
                                            if loved_json.next.is_some() {
                                                continue;
                                            }
                                        }
                                    }
                                    break;
                                }
                                // Counters
                                let mut explicit_count: u64 = 0;
                                let mut total_duration: u64 = 0;
                                let mut total_rank: u64 = 0;
                                let mut artist_count: HashMap<u64, u64> = HashMap::new();
                                let mut artist_data: HashMap<u64, models::DeezStatsTopArtist> = HashMap::new();
                                // Loop over user's loved tracks
                                for loved_track in loved {
                                    total_duration += loved_track.duration;
                                    total_rank += loved_track.rank;
                                    if loved_track.explicit {
                                        explicit_count += 1;
                                    }
                                    // Check whether artist has been seen before
                                    *artist_count.entry(loved_track.artist.id).or_insert(0) += 1;
                                    // Store artist data
                                    artist_data.insert(loved_track.artist.id, models::DeezStatsTopArtist{
                                        name: loved_track.artist.name,
                                        picture: loved_track.artist.picture_medium,
                                    });
                                }
                                // Sort and extract the top ten artists
                                let mut artist_vector: Vec<(&u64, &u64)> = artist_count.iter().collect();
                                artist_vector.sort_by(|a, b| b.1.cmp(a.1));
                                let mut top_artists: Vec<models::DeezStatsTopArtist> = Vec::new();
                                for (index, (key, _)) in artist_vector.into_iter().enumerate() {
                                    if index == 10 {
                                        break;
                                    }
                                    if let Some(artist_data) = artist_data.remove(key) {
                                        top_artists.push(artist_data);
                                    };
                                }
                                // Calculate average track duration
                                let average_duration: u64 = total_duration.checked_div(loved_index).unwrap_or(0);
                                let average = models::DeezStatsAverage{
                                    minutes: average_duration / 60,
                                    seconds: average_duration % 60,
                                };
                                // Generate the data object
                                let data = models::DeezStats{
                                    success: true,
                                    date: worker::Date::now().to_string(),
                                    user: user_json,
                                    numbers: models::DeezStatsNumbers{
                                        loved_tracks: loved_index,
                                        playlists: playlist_count,
                                        recent_tracks: history_index,
                                        explicit: explicit_count,
                                        average: average,
                                        uniqueness: models::DeezStatsUniqueness{
                                            recent: recent_rank.checked_div(history_index).unwrap_or(0),
                                            loved: total_rank.checked_div(loved_index).unwrap_or(0),
                                        },
                                    },
                                    artists: top_artists,
                                    tracks: top_tracks,
                                };
                                // Store the Deez Stats object into KV
                                kv.put(uuid, serde_json::to_string(&data)?)?.expiration_ttl(604800).execute().await?;
                                // Update statistics counter
                                let mut total_logins: u32 = 1;
                                if let Ok(Some(login_count_string)) = kv.get("_count").text().await {
                                    if let Ok(login_count) = login_count_string.parse::<u32>() {
                                        total_logins += login_count;
                                    }
                                }
                                kv.put("_count", total_logins)?.execute().await?;
                                // Return successful response
                                return Response::from_json(&json!({"success": true}));
                            }
                        }
                    }
                }
            };
            // Default fail response
            return Response::from_json(&json!({"success": false}));
        })
        .get_async("/data/:uuid", |_req: Request, ctx: RouteContext<()>| async move {
            if let Some(uuid) = ctx.param("uuid") {
                // Check if session UUID is valid
                let kv: kv::KvStore = ctx.kv("DEEZ_STATS")?;
                // If KV value is a Deez Stats data object, return it
                let data: Result<Option<models::DeezStats>, kv::KvError> = kv.get(uuid).json().await;
                if let Ok(Some(json)) = data {
                    return Response::from_json(&json);
                }
            }
            // Default fail response
            return Response::from_json(&json!({"success": false}));
        })
        .run(req, env)
        .await
}
