use std::error::Error;

#[derive(Debug)]
pub struct BuildError(Box<dyn Error>);
impl<E: Error + 'static> From<E> for BuildError {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}

pub trait Build {
    fn build(&mut self) -> Result<&mut Self, BuildError>;
}
