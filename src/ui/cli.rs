use dice::{Die, Roll, RollType};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'a', long = "advantage", conflicts_with = "disadvantage")]
    advantage: bool,

    #[arg(short = 'd', long = "disadvantage", conflicts_with = "advantage")]
    disadvantage: bool,

    /// Notación NdM (ej: 3d6, 1d20)
    pub dice: String,
}

impl Cli {
    pub fn try_parse(&self) -> Roll {
        let parts: Vec<&str> = self.dice.split('d').collect();

        if parts.len() != 2 {
            panic!("Formato inválido. Usa NdM (ej: 3d6)");
        }

        let n: usize = parts[0].parse().expect("Cantidad debe ser numérica");
        let die: usize = parts[1].parse().expect("Lados debe ser numérico");
        let die = Die::try_from(die).expect("Dado inválido");

        Roll::new_temp(n, die, self.roll_type())
    }

    fn roll_type(&self) -> RollType {
        if self.advantage {
            RollType::Advantage
        } else if self.disadvantage {
            RollType::Disadvantage
        } else {
            RollType::Normal
        }
    }
}
