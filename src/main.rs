use roll::Cli;
use roll::errors::CliError;

use clap::Parser;

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    let roll = cli.try_parse()?;
    let result = roll.roll();

    println!("{}", result);

    Ok(())
}
