use moelle_a::Sfc32;
use moelle_sample::Choice;
use seren_ex::{Context, code, reply, toolkit::Bool};
use serenity::builder::CreateEmbed;

use crate::nkolib::{self, POOL};
pub use sub::{nko_join, nko_leave, nko_status};

mod sub;

#[poise::command(slash_command)]
pub async fn nko(
    ctx: Context<'_>,
    #[description_localized("ja", "0~255")] n: Option<u8>,
    #[description_localized("ja", "結果を一次メッセージで返す")] hide: Option<Bool>,
) -> anyhow::Result<()> {
    let n = n.map(|n| n.clamp(3, 0xff - 1));

    let now = chrono::Local::now();
    let mut rng = Sfc32::from(now.timestamp_millis() as u32);

    let mut chs = vec!['-'; n.map(|n| n as usize).unwrap_or(3)];
    for ch in chs.iter_mut() {
        *ch = rng.choice(POOL).map(ToOwned::to_owned).unwrap_or('-');
    }

    let out = {
        let base: String = chs.iter().collect();

        let res_raw = nkolib::calc(&chs)?;
        let res: Vec<_> = res_raw
            .iter()
            .map(|(word, count)| format!("`{word}`: {count}"))
            .collect();

        CreateEmbed::default()
            .title(code!("{base}"))
            .field("", res.join("\n"), false)
    };

    let hide = hide.unwrap_or(Bool::False);
    ctx.send(reply!().embed(out).ephemeral(hide.into())).await?;
    Ok(())
}
