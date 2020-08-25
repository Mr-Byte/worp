use anyhow::Result;
use std::io::Write;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use worp_dice::{compiler::Compiler, runtime::machine::VirtualMachine};

fn main() -> Result<()> {
    FmtSubscriber::builder().with_max_level(Level::INFO).init();

    let mut vm = VirtualMachine::default();

    loop {
        let mut input = String::new();
        print!("Input: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        let start = std::time::Instant::now();
        let module = Compiler::compile(&input)?;

        match vm.execute(module) {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!("Result ({}s): {}", elapsed.as_secs_f64(), result);
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}
