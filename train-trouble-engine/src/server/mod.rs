use std::{net::SocketAddr, path::PathBuf};

use anyhow::Result;
use axum::{
    http::{
        header::{CACHE_CONTROL, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS},
        HeaderValue,
    },
    routing::get,
    Router,
};
use socket::socket;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer, trace::TraceLayer};
use tracing::info;

use crate::{state::ServerState, Game};

mod messages;
mod socket;

pub async fn run_server<G: Game>(
    state: ServerState<G>,
    port: u16,
    serve_path: Option<PathBuf>,
) -> Result<()> {
    let app: Router<ServerState<G>> = Router::new().route("/api/sync", get(socket::<G>));

    let app = if let Some(serve_path) = serve_path {
        app.fallback_service(
            ServiceBuilder::new()
                .layer(SetResponseHeaderLayer::if_not_present(
                    CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=600"),
                ))
                .service(ServeDir::new(serve_path)),
        )
    } else {
        app
    };

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

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    let listener = TcpListener::bind(address).await?;

    info!(%address, "Server started");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
