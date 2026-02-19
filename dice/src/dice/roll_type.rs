/// Defines how a roll is evaluated.
///
/// - `Normal`: sum of all dice.
/// - `Advantage`: roll two dice and use the highest value.
/// - `Disadvantage`: roll two dice and use the lowest value.
pub enum RollType {
    /// Make a normal roll.
    Normal,

    /// Roll two dice and use the highest result.
    Advantage,

    /// Roll two dice and use the lowest result.
    Disadvantage,
}
