use std::fs;

pub const POOL: &[char] = &['お', 'ち', 'ん', 'こ', 'ま', 'う'];

#[derive(Debug, Clone)]
pub struct Candy {
    pub word: Vec<char>,
    #[allow(unused)]
    pub point: u32,
}

pub fn load_candies() -> anyhow::Result<Vec<Candy>> {
    let f = fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/cands.csv"))?;
    let mut reader = csv::Reader::from_reader(f);

    let _ = reader.headers()?;

    let mut out = Vec::new();
    while let Some(Ok(record)) = reader.records().next() {
        if let Some(word_s) = record.get(0)
            && let Some(point_s) = record.get(1)
        {
            out.push(Candy {
                word: word_s.chars().collect(),
                point: point_s.parse()?,
            });
        }
    }
    Ok(out)
}
