mod mgr;

use std::array;

pub use mgr::{join, leave, status};
use moelle_a::Sfc32;
use moelle_sample::Choice;
use seren_ex::code;
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    http::Http,
    model::id::UserId,
};

use crate::{
    nkolib::{self, POOL},
    nkosub::mgr::load_members,
};

pub async fn broadcast(ctx: &Http) -> anyhow::Result<()> {
    for &id in load_members()?.keys() {
        let now = chrono::Local::now();
        let mut rng = Sfc32::from(now.timestamp_millis() as u32);

        let chs: [char; 5] =
            array::from_fn(|_| rng.choice(POOL).map(ToOwned::to_owned).unwrap_or('*'));

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

        let user = ctx.get_user(UserId::new(id)).await?;
        user.dm(ctx, CreateMessage::new().embed(out)).await?;
    }

    Ok(())
}
