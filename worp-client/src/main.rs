use anyhow::Result;
use std::io::Write;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use worp_dice::Dice;

fn main() -> Result<()> {
    FmtSubscriber::builder().with_max_level(Level::INFO).init();

    // TODO: Create a system that handles runtime, plus parsing and execution of scripts as a unit.

    let mut runtime = Dice::default();

    loop {
        let mut input = String::new();
        print!("Input: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        let start = std::time::Instant::now();

        match runtime.disassemble_script(&input) {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!("Result ({} ms): {}", (elapsed.as_micros() as f64 / 1000.0), result);
            }
            Err(err) => eprintln!("{}", err),
        };
    }
}
