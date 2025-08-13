use anyhow::Result;

fn main() -> Result<()> {
    println!("owg-server starting (Sprint A stub)...");
    owg_sim::run_headless_example()?;
    println!("owg-server exiting.");
    Ok(())
}
