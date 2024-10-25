use crate::{
    channel::{try_from_i2c_data, Channel},
    chip_definitions::{
        DataConfig, GeneralConfigFlags, OpCode, Oversampling, RegisterAddress, SamplingRate,
        SequenceConfig, SystemStatusFlags,
    },
    error::Error,
};
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub(crate) struct ChipInterface<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> ChipInterface<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Error<I2C::Error>>,
{
    pub(crate) fn new(i2c: I2C, address: u8) -> Self {
        ChipInterface { i2c, address }
    }

    pub(crate) fn configure_all_pins_as_analog_inputs(&mut self) -> Result<(), Error<I2C::Error>> {
        // Turn off auto-sequence mode
        self.write_sequence_config(SequenceConfig::Manual)?;

        // Clear GPIO configuration
        // Not absolutely necessary, as PinConfig of 0 overrides these values.
        self.register_write(RegisterAddress::GpioConfig, 0b_0000_0000)?;
        self.register_write(RegisterAddress::GpioDriveConfig, 0b_0000_0000)?;

        // Configure pins as analog inputs
        self.register_write(RegisterAddress::PinConfig, 0b_0000_0000)
    }

    pub(crate) fn configure_oversampling(
        &mut self,
        ratio: Oversampling,
    ) -> Result<(), Error<I2C::Error>> {
        self.register_write(RegisterAddress::OsrConfig, ratio.value())
    }

    pub(crate) fn configure_sampling_rate(
        &mut self,
        config: SamplingRate,
    ) -> Result<(), Error<I2C::Error>> {
        self.register_write(RegisterAddress::OpModeConfig, config.value())
    }

    pub(crate) fn set_channel(&mut self, channel: Channel) -> Result<(), Error<I2C::Error>> {
        self.register_write(RegisterAddress::ChannelSelect, channel as u8)
    }

    pub(crate) fn configure_auto_sequence_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_data_config(DataConfig::NormalDataAddChannelID)?;
        self.write_sequence_config(SequenceConfig::StoppedAuto)?;
        self.register_write(RegisterAddress::AutoSequenceChannelSelect, 0b_1111_1111)
    }

    pub(crate) fn configure_manual_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_data_config(DataConfig::NormalDataAddChannelID)?;
        self.write_sequence_config(SequenceConfig::Manual)
    }

    pub(crate) fn read_system_status(&mut self) -> Result<SystemStatusFlags, Error<I2C::Error>> {
        let bits = self.register_read(RegisterAddress::SystemStatus)?;
        Ok(SystemStatusFlags::from_bits_retain(bits))
    }

    pub(crate) fn read_general_config(&mut self) -> Result<GeneralConfigFlags, Error<I2C::Error>> {
        let bits = self.register_read(RegisterAddress::GeneralConfig)?;
        Ok(GeneralConfigFlags::from_bits_retain(bits))
    }

    pub(crate) fn write_general_config(
        &mut self,
        config: GeneralConfigFlags,
    ) -> Result<(), Error<I2C::Error>> {
        self.register_write(RegisterAddress::DataConfig, config.bits())
    }

    pub(crate) fn write_data_config(
        &mut self,
        config: DataConfig,
    ) -> Result<(), Error<I2C::Error>> {
        self.register_write(RegisterAddress::DataConfig, config.value())
    }

    pub(crate) fn write_sequence_config(
        &mut self,
        config: SequenceConfig,
    ) -> Result<(), Error<I2C::Error>> {
        self.register_write(RegisterAddress::SequenceConfig, config.value())
    }

    fn register_read(&mut self, r: RegisterAddress) -> Result<u8, Error<I2C::Error>> {
        let mut incoming = [0_u8; 1];
        let res = self.i2c.write_read(
            self.address,
            &[OpCode::SingleRegisterRead.value(), r.value()],
            &mut incoming,
        );

        Ok(res.map(|()| incoming[0])?)
    }

    fn register_write(&mut self, r: RegisterAddress, val: u8) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[OpCode::SingleRegisterWrite.value(), r.value(), val],
        )?;
        Ok(())
    }

    pub(crate) fn data_read(&mut self) -> Result<[u16; 8], Error<I2C::Error>> {
        let mut data_buffer = [0_u8; (8 * 3)];
        if let Err(err) = self.i2c.read(self.address, &mut data_buffer) {
            return Err(Error::I2cError(err));
        }

        let mut out = [0_u16; 8];
        data_buffer
            .chunks_exact(3)
            .zip(out.iter_mut())
            .for_each(|(chunk, destination)| {
                let mut buf = [0_u8; 2];

                #[allow(
                    clippy::indexing_slicing,
                    reason = "Safe due to slice matching buf size"
                )]
                buf.copy_from_slice(&chunk[..2]);
                *destination = u16::from_be_bytes(buf);
            });
        Ok(out)
    }

    pub(crate) fn data_channel_read(
        &mut self,
        desired_channel: Channel,
    ) -> Result<(u16, usize), Error<I2C::Error>> {
        const MAX_CHANNEL_READ_TRIES: usize = 32;
        for i in 0..MAX_CHANNEL_READ_TRIES {
            let mut data_buffer = [0_u8; 3];
            if let Err(err) = self.i2c.read(self.address, &mut data_buffer) {
                return Err(Error::I2cError(err));
            }

            let read_channel = try_from_i2c_data(data_buffer[2])?;

            if read_channel == desired_channel {
                let mut buf = [0_u8; 2];

                #[allow(
                    clippy::indexing_slicing,
                    reason = "Safe due to slice matching buf size"
                )]
                buf.copy_from_slice(&data_buffer[..2]);

                return Ok((u16::from_be_bytes(buf), i + 1));
            }
        }
        Err(Error::IncorrectChannelAddress)
    }
}
