use roll::Cli;

use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let roll = cli.try_parse();
    let result = roll.roll();

    println!("{}", result);
}
