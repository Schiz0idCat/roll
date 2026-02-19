use dice::Rollable;
use roll::Cli;

use clap::Parser;

use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.try_parse() {
        Ok(rolls) => {
            println!("{}", rolls.roll());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
