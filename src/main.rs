pub mod execution_driver;
pub mod executors;
pub mod setup_day;
pub mod utils;

use clap::{Parser, Subcommand};
use setup_day::setup_day;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Fetch { day: u8 },
    Execute { day: u8 },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Fetch { day } => {
            setup_day(day);
        }
        Command::Execute { day } => {
            let executors = executors::get_executors();
            execution_driver::execute(executors, day);
        }
    }
}
