use core::convert::TryFrom;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
    Channel0 = 0x00,
    Channel1 = 0x01,
    Channel2 = 0x02,
    Channel3 = 0x03,
    Channel4 = 0x04,
    Channel5 = 0x05,
    Channel6 = 0x06,
    Channel7 = 0x07,
}

impl TryFrom<u8> for Channel {
    type Error = ();

    fn try_from(value: u8) -> Result<Channel, Self::Error> {
        match value {
            x if x == Channel::Channel0 as u8 => Ok(Channel::Channel0),
            x if x == Channel::Channel1 as u8 => Ok(Channel::Channel1),
            x if x == Channel::Channel2 as u8 => Ok(Channel::Channel2),
            x if x == Channel::Channel3 as u8 => Ok(Channel::Channel3),
            x if x == Channel::Channel4 as u8 => Ok(Channel::Channel4),
            x if x == Channel::Channel5 as u8 => Ok(Channel::Channel5),
            x if x == Channel::Channel6 as u8 => Ok(Channel::Channel6),
            x if x == Channel::Channel7 as u8 => Ok(Channel::Channel7),
            _ => Err(()),
        }
    }
}

pub(crate) fn try_from_i2c_data<E>(value: u8) -> Result<Channel, Error<E>> {
    match value >> 4 {
        x if x == Channel::Channel0 as u8 => Ok(Channel::Channel0),
        x if x == Channel::Channel1 as u8 => Ok(Channel::Channel1),
        x if x == Channel::Channel2 as u8 => Ok(Channel::Channel2),
        x if x == Channel::Channel3 as u8 => Ok(Channel::Channel3),
        x if x == Channel::Channel4 as u8 => Ok(Channel::Channel4),
        x if x == Channel::Channel5 as u8 => Ok(Channel::Channel5),
        x if x == Channel::Channel6 as u8 => Ok(Channel::Channel6),
        x if x == Channel::Channel7 as u8 => Ok(Channel::Channel7),
        _ => Err(Error::InvalidChannelAddress),
    }
}
