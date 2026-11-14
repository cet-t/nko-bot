use std::{
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
};

const PATH: &str = "./subs.csv";

pub fn join(id: u64) -> anyhow::Result<()> {
    let f = fs::File::open(self::PATH)?;
    let mut w = BufWriter::new(f);

    let ts = chrono::Utc::now().timestamp();
    writeln!(w, "+,{id},{ts}")?;

    Ok(())
}

pub fn leave(id: u64) -> anyhow::Result<()> {
    let f = fs::File::open(self::PATH)?;
    let mut w = BufWriter::new(f);

    let ts = chrono::Utc::now().timestamp();
    writeln!(w, "-,{id},{ts}")?;

    Ok(())
}

pub fn status(id: u64) -> Option<i64> {
    let mems = self::load_members().ok()?;
    mems.get(&id).copied()
}

pub fn load_members() -> anyhow::Result<HashMap<u64, i64>> {
    let mut mems = HashMap::new();

    let f = fs::File::open(PATH)?;
    let mut r = csv::Reader::from_reader(f);
    let _ = r.headers()?;

    while let Some(Ok(record)) = r.records().next() {
        if let Some(op) = record.get(0)
            && let Some(id) = record.get(1).and_then(|id| id.parse::<u64>().ok())
            && let Some(ts) = record.get(2).and_then(|ts| ts.parse::<i64>().ok())
        {
            match op {
                "+" => {
                    mems.insert(id, ts);
                }
                "-" => {
                    mems.remove(&id);
                }
                _ => unreachable!(),
            }
        }
    }

    self::compact(&mems)?;

    Ok(mems)
}

fn compact(mems: &HashMap<u64, i64>) -> anyhow::Result<()> {
    let f = fs::File::create(PATH)?;
    let mut w = BufWriter::new(f);
    mems.iter()
        .try_for_each(|(id, ts)| writeln!(w, "+,{id},{ts}"))?;

    Ok(())
}
