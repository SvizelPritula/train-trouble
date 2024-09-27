use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use tokio::time::interval;

use crate::{
    save::{spawn_save, SAVE_INTERVAL},
    state::{Action, ServerState},
    Game,
};

pub async fn run_loop<G: Game>(
    mut game: G,
    state: ServerState<G>,
    save_path: Option<PathBuf>,
) -> Result<()> {
    let period = Duration::from_secs(1) / G::TICK_RATE.try_into().unwrap();
    let mut game_interval = interval(period);

    let ticks_per_save: u64 = SAVE_INTERVAL.as_secs() * G::TICK_RATE;
    let mut ticks_until_save: u64 = ticks_per_save;

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

        game_interval.tick().await;
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

        ticks_until_save -= 1;
        if ticks_until_save == 0 {
            spawn_save(game.clone(), save_path.clone());
            ticks_until_save = ticks_per_save;
        }
    }
}
