use crate::chip_definitions::{
    DataConfig, GeneralConfigFlags, OpCode, Oversampling, RegisterAddress, SequenceConfig,
    SystemStatusFlags,
};
use embedded_hal::i2c::{ErrorType, I2c, SevenBitAddress};

// #[allow(clippy::exhaustive_enums)]
// #[derive(Debug)]
// pub enum Tla2528Error<E: ErrorType> {
//     I2cError(E::Error),
// }

pub(crate) struct ChipInterface<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> ChipInterface<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    pub(crate) fn new(i2c: I2C, address: u8) -> Self {
        ChipInterface { i2c, address }
    }

    pub(crate) fn configure_all_pins_as_analog_inputs(
        &mut self,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::PinConfig, 0b_0000_0000)?;

        // Not absolutely necessary, as PinConfig of 0 overrides these values.
        self.register_write(RegisterAddress::GpioConfig, 0b_0000_0000)?;
        self.register_write(RegisterAddress::GpioDriveConfig, 0b_0000_0000)
    }

    pub(crate) fn configure_oversampling(
        &mut self,
        ratio: Oversampling,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.write_oversampling_config(ratio)
    }

    pub(crate) fn configure_auto_sequence_mode(&mut self) -> Result<(), <I2C as ErrorType>::Error> {
        self.write_data_config(DataConfig::NormalDataAddChannelID)?;
        self.write_sequence_config(SequenceConfig::StoppedAuto)?;
        self.register_write(RegisterAddress::AutoSequenceChannelSelect, 0b_1111_1111)
    }

    pub(crate) fn read_system_status(
        &mut self,
    ) -> Result<SystemStatusFlags, <I2C as ErrorType>::Error> {
        let bits = self.register_read(RegisterAddress::SystemStatus)?;
        Ok(SystemStatusFlags::from_bits_retain(bits))
    }

    pub(crate) fn read_general_config(
        &mut self,
    ) -> Result<GeneralConfigFlags, <I2C as ErrorType>::Error> {
        let bits = self.register_read(RegisterAddress::GeneralConfig)?;
        Ok(GeneralConfigFlags::from_bits_retain(bits))
    }

    pub(crate) fn write_general_config(
        &mut self,
        config: GeneralConfigFlags,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::DataConfig, config.bits())
    }

    pub(crate) fn write_data_config(
        &mut self,
        config: DataConfig,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::DataConfig, config.value())
    }

    pub(crate) fn write_oversampling_config(
        &mut self,
        config: Oversampling,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::OsrConfig, config.value())
    }

    pub(crate) fn write_sequence_config(
        &mut self,
        config: SequenceConfig,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::SequenceConfig, config.value())
    }

    fn register_read(&mut self, r: RegisterAddress) -> Result<u8, <I2C as ErrorType>::Error> {
        let mut incoming = [0; 1];
        match self.i2c.write_read(
            self.address,
            &[OpCode::SingleRegisterRead.value(), r.value()],
            &mut incoming,
        ) {
            Ok(()) => Ok(incoming[0]),
            Err(err) => Err(err),
        }
    }

    fn register_write(
        &mut self,
        r: RegisterAddress,
        val: u8,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.i2c.write(
            self.address,
            &[OpCode::SingleRegisterWrite.value(), r.value(), val],
        )
    }

    pub(crate) fn data_read(&mut self) -> Result<[u16; 8], <I2C as ErrorType>::Error> {
        // Would prefer to work with an array of these,
        // packed so the struct was exactly 3 bytes
        // struct AdcOutput {
        //     data: u16,
        //     channel: u8,
        // }
        let mut data_buffer = [0_u8; (8 * 3)];
        self.i2c.read(self.address, &mut data_buffer)?;
        #[allow(clippy::cast_lossless)]
        let data = [
            (256_u16 * data_buffer[0] as u16) + data_buffer[1] as u16,
            (256_u16 * data_buffer[3] as u16) + data_buffer[4] as u16,
            (256_u16 * data_buffer[6] as u16) + data_buffer[7] as u16,
            (256_u16 * data_buffer[9] as u16) + data_buffer[10] as u16,
            (256_u16 * data_buffer[12] as u16) + data_buffer[13] as u16,
            (256_u16 * data_buffer[15] as u16) + data_buffer[16] as u16,
            (256_u16 * data_buffer[18] as u16) + data_buffer[19] as u16,
            (256_u16 * data_buffer[21] as u16) + data_buffer[22] as u16,
        ];
        Ok(data)
    }
}
