use crate::chip_definitions::{
    DataConfig, GeneralConfigFlags, OpCode, Oversampling, RegisterAddress, SamplingRate,
    SequenceConfig, SystemStatusFlags,
};
use embedded_hal::i2c::{ErrorType, I2c, SevenBitAddress};

#[derive(Debug)]
#[non_exhaustive]
pub enum Tla2528Error<E> {
    I2cError(E),
    _DataItemsMisOrdered,
}

impl<E> From<E> for Tla2528Error<E> {
    fn from(other: E) -> Self {
        Tla2528Error::I2cError(other)
    }
}

pub(crate) struct ChipInterface<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> ChipInterface<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Tla2528Error<I2C::Error>>,
{
    pub(crate) fn new(i2c: I2C, address: u8) -> Self {
        ChipInterface { i2c, address }
    }

    pub(crate) fn configure_all_pins_as_analog_inputs(
        &mut self,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.register_write(RegisterAddress::PinConfig, 0b_0000_0000)?;

        // Not absolutely necessary, as PinConfig of 0 overrides these values.
        self.register_write(RegisterAddress::GpioConfig, 0b_0000_0000)?;
        self.register_write(RegisterAddress::GpioDriveConfig, 0b_0000_0000)
    }

    pub(crate) fn configure_oversampling(
        &mut self,
        ratio: Oversampling,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.register_write(RegisterAddress::OsrConfig, ratio.value())
    }

    pub(crate) fn configure_sampling_rate(
        &mut self,
        config: SamplingRate,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.register_write(RegisterAddress::OpModeConfig, config.value())
    }

    pub(crate) fn configure_auto_sequence_mode(&mut self) -> Result<(), Tla2528Error<I2C::Error>> {
        self.write_data_config(DataConfig::NormalDataAddChannelID)?;
        self.write_sequence_config(SequenceConfig::StoppedAuto)?;
        self.register_write(RegisterAddress::AutoSequenceChannelSelect, 0b_1111_1111)
    }

    pub(crate) fn read_system_status(
        &mut self,
    ) -> Result<SystemStatusFlags, Tla2528Error<I2C::Error>> {
        let bits = self.register_read(RegisterAddress::SystemStatus)?;
        Ok(SystemStatusFlags::from_bits_retain(bits))
    }

    pub(crate) fn read_general_config(
        &mut self,
    ) -> Result<GeneralConfigFlags, Tla2528Error<I2C::Error>> {
        let bits = self.register_read(RegisterAddress::GeneralConfig)?;
        Ok(GeneralConfigFlags::from_bits_retain(bits))
    }

    pub(crate) fn write_general_config(
        &mut self,
        config: GeneralConfigFlags,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.register_write(RegisterAddress::DataConfig, config.bits())
    }

    pub(crate) fn write_data_config(
        &mut self,
        config: DataConfig,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.register_write(RegisterAddress::DataConfig, config.value())
    }

    pub(crate) fn write_sequence_config(
        &mut self,
        config: SequenceConfig,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.register_write(RegisterAddress::SequenceConfig, config.value())
    }

    fn register_read(&mut self, r: RegisterAddress) -> Result<u8, Tla2528Error<I2C::Error>> {
        let mut incoming = [0_u8; 1];
        let res = self.i2c.write_read(
            self.address,
            &[OpCode::SingleRegisterRead.value(), r.value()],
            &mut incoming,
        );

        Ok(res.map(|()| incoming[0])?)
    }

    fn register_write(
        &mut self,
        r: RegisterAddress,
        val: u8,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.i2c.write(
            self.address,
            &[OpCode::SingleRegisterWrite.value(), r.value(), val],
        )?;
        Ok(())
    }

    pub(crate) fn data_read(&mut self) -> Result<[u16; 8], <I2C as ErrorType>::Error> {
        let mut data_buffer = [0_u8; (8 * 3)];
        self.i2c.read(self.address, &mut data_buffer)?;

        let mut out = [0_u16; 8];
        data_buffer
            .chunks_exact(3)
            .zip(out.iter_mut())
            .for_each(|(chunk, destination)| {
                let mut buf = [0_u8; 2];

                #[allow(clippy::indexing_slicing)] // Safe due to slice matching buf size
                buf.copy_from_slice(&chunk[..2]);
                *destination = u16::from_be_bytes(buf);
            });
        Ok(out)
    }
}
