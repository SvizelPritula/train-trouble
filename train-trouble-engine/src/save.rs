use std::{
    ffi::OsString,
    io::ErrorKind,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;
use tokio::{fs, spawn};
use tracing::error;

use crate::Game;

pub const TEMP_SUFFIX: &str = ".tmp";
pub const SAVE_INTERVAL: Duration = Duration::from_secs(10);

pub async fn load<G: Game>(path: &Option<PathBuf>) -> Result<G> {
    let Some(path) = path else {
        return Ok(G::default());
    };

    match fs::read(path).await {
        Ok(payload) => Ok(serde_json::from_slice(&payload)?),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(G::default()),
            _ => Err(error.into()),
        },
    }
}

pub async fn save<G: Game>(game: G, path: &Path) -> Result<()> {
    let mut temp_path = path.as_os_str().to_owned();
    temp_path.push(OsString::from(TEMP_SUFFIX));

    let payload = serde_json::to_vec(&game)?;
    fs::write(&temp_path, payload).await?;

    fs::rename(temp_path, path).await?;
    Ok(())
}

pub fn spawn_save<G: Game>(game: G, path: Option<PathBuf>) {
    if let Some(path) = path {
        spawn(async move {
            match save(game, &path).await {
                Ok(()) => (),
                Err(error) => error!("Failed to save game: {error}"),
            }
        });
    }
}
