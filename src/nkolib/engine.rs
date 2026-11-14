use std::collections::HashMap;

use crate::nkolib::candy::load_candies;

/// "おちんこちんこ" -> ("おちんちん", 1), ("ちんちん", 1), ("おちんこ", 1), ("ちんこ", 2)
pub fn calc(chars: &[char]) -> anyhow::Result<HashMap<String, usize>> {
    if chars.is_empty() {
        anyhow::bail!("chars is empty")
    }

    let mut available = HashMap::new();
    for &ch in chars {
        *available.entry(ch).or_insert(0usize) += 1;
    }

    let mut out = HashMap::new();

    load_candies()?.into_iter().for_each(|candy| {
        let mut required = HashMap::new();
        candy.word.iter().for_each(|&ch| {
            required.entry(ch).and_modify(|i| *i += 1).or_insert(1usize);
        });

        let count = required
            .iter()
            .map(|(ch, needed)| available.get(ch).copied().unwrap_or(0) / needed)
            .min()
            .unwrap_or(0);

        if count > 0 {
            out.insert(candy.word.iter().collect(), count);
        }
    });

    Ok(out)
}
