use std::error::Error;

#[derive(Debug)]
pub struct ExecError(Box<dyn Error>);
impl<E: Error + 'static> From<E> for ExecError {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}

pub trait Exec {
    fn exec(&self) -> Result<&Self, ExecError>;
}
