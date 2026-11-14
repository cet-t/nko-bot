use std::{str::FromStr, time::Duration};

use poise::FrameworkOptions;
use seren_ex::{
    CommandRegistration, CommonCommands, FrameworkConfig, PresenceOptions, SetupOptions,
    StartOptions,
};
use serenity::{gateway::ActivityData, model::user::OnlineStatus, prelude::GatewayIntents};

mod commands;
mod nkolib;
mod nkosub;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let intents = GatewayIntents::all();

    seren_ex::start(StartOptions {
        token_key: Some("TOKEN"),
        intents,
        framework: FrameworkConfig {
            options: FrameworkOptions {
                commands: vec![
                    commands::nko(),
                    commands::nko_join(),
                    commands::nko_leave(),
                    commands::nko_status(),
                ],
                ..Default::default()
            },
            setup: SetupOptions {
                presence: Some(PresenceOptions {
                    activity: Some(ActivityData::competing("comp")),
                    status: OnlineStatus::Online,
                }),
                registration: CommandRegistration::Global,
                log_ready: true,
                custom: None,
            }
            .setup(|ctx, ready, _fw| {
                Box::pin(async move {
                    println!("{} is ready", ready.user.name);

                    let http = ctx.http.clone();
                    tokio::spawn(async move {
                        let schedule = cron::Schedule::from_str("* * 9,21 * * * *").unwrap();
                        let mut next_tick = schedule.upcoming(chrono::Utc).next().unwrap();
                        loop {
                            let now = chrono::Utc::now();
                            if now >= next_tick {
                                let _ = nkosub::broadcast(&http).await;

                                next_tick = schedule.upcoming(chrono::Utc).next().unwrap();
                            }

                            tokio::time::sleep(Duration::from_secs(
                                (next_tick - now).num_seconds().min(60) as u64,
                            ))
                            .await;
                        }
                    });

                    Ok(())
                })
            }),
            common_commands: Some(CommonCommands::all()),
        },
    })
    .await?;

    Ok(())
}
