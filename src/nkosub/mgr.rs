use std::{
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
};

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/subs.csv");
pub const JOIN: &str = "+";
pub const LEAVE: &str = "-";

pub fn join(id: u64) -> anyhow::Result<()> {
    let f = fs::OpenOptions::new().append(true).open(self::PATH)?;
    let mut w = BufWriter::new(f);

    let ts = chrono::Utc::now().timestamp();
    writeln!(w, "{JOIN},{id},{ts}")?;
    w.flush()?;

    Ok(())
}

pub fn leave(id: u64) -> anyhow::Result<()> {
    let f = fs::OpenOptions::new().append(true).open(self::PATH)?;
    let mut w = BufWriter::new(f);

    let ts = chrono::Utc::now().timestamp();
    writeln!(w, "{LEAVE},{id},{ts}")?;
    w.flush()?;

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
                JOIN => {
                    mems.insert(id, ts);
                }
                LEAVE => {
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
    writeln!(w, "op,id,ts")?;
    mems.iter()
        .try_for_each(|(id, ts)| writeln!(w, "{JOIN},{id},{ts}"))?;
    w.flush()?;

    Ok(())
}
