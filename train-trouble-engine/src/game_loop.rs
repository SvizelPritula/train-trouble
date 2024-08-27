use std::time::Duration;

use tokio::time::interval;

use crate::{state::StateHandle, Game};

pub async fn run_loop<G: Game>(state: StateHandle<G>) {
    let mut game = G::default();

    let period = Duration::from_secs(1) / G::TICK_RATE.try_into().unwrap();
    let mut interval = interval(period);

    loop {
        interval.tick().await;
        game.tick();

        {
            let guard = state.lock();

            for (channel, sender) in guard.subscriptions() {
                let view = game.view(channel.clone());

                sender.send_if_modified(|value| {
                    if !value.as_ref().is_some_and(|value| value == &view) {
                        *value = Some(view);
                        true
                    } else {
                        false
                    }
                });
            }
        }
    }
}
