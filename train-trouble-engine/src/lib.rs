use anyhow::{Error, Result};
use clap::Parser;
use game_loop::run_loop;
use save::load;
use serde::{Deserialize, Serialize};
use server::run_server;
use state::ServerState;
use std::{future::Future, hash::Hash, path::PathBuf};
use tokio::try_join;
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer};

mod channel_map;
mod game_loop;
mod save;
mod server;
mod state;

pub trait Game: Serialize + for<'de> Deserialize<'de> + Clone + Default + Send + 'static {
    type CHANNEL: for<'de> Deserialize<'de> + Clone + Eq + Hash + Send + Sync;
    type VIEW: Serialize + Clone + Eq + Send + Sync;
    type ACTION: for<'de> Deserialize<'de> + Clone + Eq + Send + Sync;

    const TICK_RATE: u64;

    fn tick(&mut self);
    fn view(&mut self, channel: Self::CHANNEL) -> Self::VIEW;
    fn act(&mut self, channel: Self::CHANNEL, action: Self::ACTION) -> ActionResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
    Ok,
    Error(Box<str>),
    Misdirected,
}

#[derive(Debug, Clone, Parser)]
struct CliArgs {
    /// The port to listen on
    #[arg(long, default_value_t = 8000, env)]
    port: u16,
    /// A directory to serve at server root
    #[arg(long, env)]
    serve: Option<PathBuf>,
    /// The path to a file in which the game will be saved
    #[arg(long, env)]
    save: Option<PathBuf>,
}

#[tokio::main]
pub async fn run<G: Game + 'static>() -> Result<()> {
    let args = CliArgs::parse();

    let stdout = fmt::layer().with_filter(LevelFilter::INFO);
    registry().with(stdout).init();

    let game: G = load(&args.save)
        .await
        .inspect_err(|error| error!("Failed to load save: {error}"))?;

    let state_for_loop = ServerState::<G>::default();
    let state_for_server = state_for_loop.clone();

    try_join!(
        spawn_anyhow(|| run_loop(game, state_for_loop, args.save)),
        spawn_anyhow(|| run_server(state_for_server, args.port, args.serve)),
    )
    .inspect_err(|error| {
        error!("Failed to run server: {error}");
    })
    .map(|((), ())| ())
}

async fn spawn_anyhow<F, P, R>(f: F) -> Result<R>
where
    F: FnOnce() -> P,
    P: Future<Output = Result<R>> + Send + 'static,
    R: Send + 'static,
{
    tokio::spawn(f())
        .await
        .unwrap_or_else(|e| Err(Error::from(e)))
}
