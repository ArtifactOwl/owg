use anyhow::Result;
use owg_protocol::State;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn save_json<P: AsRef<Path>>(path: P, state: &State) -> Result<()> {
    let s = serde_json::to_string_pretty(state)?;
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())?;
    Ok(())
}
