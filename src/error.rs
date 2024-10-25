#[derive(Debug)]
#[non_exhaustive]
pub enum Error<E> {
    I2cError(E),
    _DataItemsMisOrdered,
    IncorrectChannelAddress,
    InvalidChannelAddress,
}

impl<E> From<E> for Error<E> {
    fn from(other: E) -> Self {
        Error::I2cError(other)
    }
}
