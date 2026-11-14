use seren::TimestampConvert;
use seren_ex::{Context, reply};

use crate::nkosub;

#[poise::command(slash_command, name_localized("ja", "んこサブスク入会"))]
pub async fn nko_join(ctx: Context<'_>) -> anyhow::Result<()> {
    nkosub::join(ctx.author().id.get())?;

    ctx.send(reply!("入会しました").ephemeral(true)).await?;

    Ok(())
}

#[poise::command(slash_command, name_localized("ja", "んこサブスク退会"))]
pub async fn nko_leave(ctx: Context<'_>) -> anyhow::Result<()> {
    nkosub::leave(ctx.author().id.get())?;

    ctx.send(reply!("退会しました").ephemeral(true)).await?;

    Ok(())
}

#[poise::command(slash_command, name_localized("ja", "んこサブスク"))]
pub async fn nko_status(ctx: Context<'_>) -> anyhow::Result<()> {
    let r = if let Some(ts) = nkosub::status(ctx.author().id.get()) {
        let dt = |ts_ms: i64| chrono::DateTime::from_timestamp(ts_ms, 0).map(|dt| dt.naive_local());

        reply!(
            "{}",
            dt(ts).map(|ndt| ndt.to_relative()).unwrap_or_default()
        )
    } else {
        reply!("未登録")
    };
    ctx.send(r.ephemeral(true)).await?;

    Ok(())
}
