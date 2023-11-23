use std::error::Error;

#[derive(Debug)]
pub struct LoadError(Box<dyn Error>);
impl<E: Error + 'static> From<E> for LoadError {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}

pub trait Load {
    fn load(&mut self) -> Result<&mut Self, LoadError>;
}
