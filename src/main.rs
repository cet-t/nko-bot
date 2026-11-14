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
            },
            common_commands: Some(CommonCommands::all()),
        },
    })
    .await?;

    Ok(())
}
