use std::time::Duration;

use anyhow::Result;
use tokio::time::interval;

use crate::{
    state::{Action, ServerState},
    Game,
};

pub async fn run_loop<G: Game>(state: ServerState<G>) -> Result<()> {
    let mut game = G::default();

    let period = Duration::from_secs(1) / G::TICK_RATE.try_into().unwrap();
    let mut interval = interval(period);

    loop {
        let actions = state.actions.take_actions();

        for Action {
            channel,
            action,
            sender,
        } in actions
        {
            let result = game.act(channel, action);
            let _ = sender.send(result);
        }

        interval.tick().await;
        game.tick();

        {
            let guard = state.subscriptions.subscriptions();

            for (channel, sender) in guard.iter() {
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
