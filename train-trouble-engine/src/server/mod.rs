use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    http::{
        header::{REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS},
        HeaderValue,
    },
    routing::get,
    Router,
};
use socket::socket;
use tokio::net::TcpListener;
use tower_http::{set_header::SetResponseHeaderLayer, trace::TraceLayer};
use tracing::info;

use crate::{state::ServerState, Game};

mod messages;
mod socket;

pub async fn run_server<G: Game>(state: ServerState<G>) -> Result<()> {
    let app = Router::new()
        .route("/api/sync", get(socket))
        .route("/", get(|| async { "Hello, world!" }));

    let app = app
        .layer(SetResponseHeaderLayer::if_not_present(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(address).await?;

    info!(%address, "Server started");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
