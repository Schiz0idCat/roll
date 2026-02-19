pub trait Rollable {
    type Output;

    fn roll(&self) -> Self::Output;
}
