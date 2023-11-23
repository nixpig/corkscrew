use std::error::Error;

#[derive(Debug)]
pub struct ParseError(Box<dyn Error>);
impl<E: Error + 'static> From<E> for ParseError {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}

pub trait Parse {
    fn parse(&mut self) -> Result<&mut Self, ParseError>;
}
