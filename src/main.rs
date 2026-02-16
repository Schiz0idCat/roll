use clap::Parser;
use dice::{Die, Roll};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short = 'a', long = "advantage")]
    advantage: bool,

    #[arg(short = 'd', long = "disadvantage")]
    disadvantage: bool,

    /// Notación NdM (ej: 3d6, 1d20)
    dice: String,
}

impl Cli {
    fn parse_dice(input: &str) -> (usize, Die) {
        let parts: Vec<&str> = input.split('d').collect();

        if parts.len() != 2 {
            panic!("Formato inválido. Usa NdM (ej: 3d6)");
        }

        let n: usize = parts[0].parse().expect("Cantidad debe ser numérica");
        let dice: usize = parts[1].parse().expect("Lados debe ser numérico");
        let dice = Die::try_from(dice).expect("Dado inválido");

        (n, dice)
    }
}

fn main() {
    let cli = Cli::parse();
    let (n, die) = Cli::parse_dice(&cli.dice);

    if cli.advantage && cli.disadvantage {
        panic!("No puedes usar ventaja y desventaja al mismo tiempo");
    }

    let result = if cli.advantage {
        Roll::roll_advantage(die)
    } else if cli.disadvantage {
        Roll::roll_disadvantage(die)
    } else {
        Roll::roll_n(n, die)
    };

    println!("Rolls: {:?}", result.rolls());
    println!("Total: {}", result.total());
}
