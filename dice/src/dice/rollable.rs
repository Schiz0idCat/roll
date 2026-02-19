/// A trait for types that can be executed as a dice roll.
///
/// Implementors produce an associated `Output` when rolled.
/// The output type depends on the concrete roll structure.
pub trait Rollable {
    type Output;

    /// Executes the roll and returns its result.
    fn roll(&self) -> Self::Output;
}
