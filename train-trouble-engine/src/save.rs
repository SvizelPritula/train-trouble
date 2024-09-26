use std::{io::ErrorKind, time::Duration};

use anyhow::Result;
use tokio::{fs, spawn};
use tracing::error;

use crate::Game;

pub const SAVE_PATH: &str = "./save.json";
pub const SAVE_PATH_TEMP: &str = "./save.json.tmp";
pub const SAVE_INTERVAL: Duration = Duration::from_secs(10);

pub async fn load<G: Game>() -> Result<G> {
    match fs::read(SAVE_PATH).await {
        Ok(payload) => Ok(serde_json::from_slice(&payload)?),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(G::default()),
            _ => Err(error.into()),
        },
    }
}

pub async fn save<G: Game>(game: G) -> Result<()> {
    let payload = serde_json::to_vec(&game)?;
    fs::write(SAVE_PATH_TEMP, payload).await?;

    fs::rename(SAVE_PATH_TEMP, SAVE_PATH).await?;
    Ok(())
}

pub fn spawn_save<G: Game>(game: G) {
    spawn(async {
        match save(game).await {
            Ok(()) => (),
            Err(error) => error!("Failed to save game: {error}"),
        }
    });
}
