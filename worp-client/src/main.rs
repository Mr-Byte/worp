use anyhow::Result;
use std::io::Write;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use worp_dice::{compiler::Compiler, runtime::Runtime};

fn main() -> Result<()> {
    FmtSubscriber::builder().with_max_level(Level::INFO).init();

    // TODO: Create a system that handles runtime, plus parsing and execution of scripts as a unit.

    let mut runtime = Runtime::default();

    // loop {
    let mut input = String::from("let mut x = 0; while x < 100000000 { x = x + 1; }; x");
    // print!("Input: ");
    // std::io::stdout().flush()?;
    // std::io::stdin().read_line(&mut input)?;

    let start = std::time::Instant::now();
    let script = Compiler::compile_script(&input)?;

    match runtime.run_script(script) {
        Ok(result) => {
            let elapsed = start.elapsed();
            println!("Result ({} ms): {}", (elapsed.as_micros() as f64 / 1000.0), result);
        }
        Err(err) => eprintln!("{}", err),
    };

    Ok(())
    // }
}
