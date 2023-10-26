use crate::chip_definitions::{
    DataConfig, GeneralConfigFlags, OpCode, Oversampling, RegisterAddress, SequenceConfig,
    SystemStatusFlags,
};
use embedded_hal_async::i2c::{ErrorType, I2c, SevenBitAddress};

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

    pub(crate) async fn configure_all_pins_as_analog_inputs(
        &mut self,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::PinConfig, 0b_0000_0000)
            .await?;

        // Not absolutely necessary, as PinConfig of 0 overrides these values.
        self.register_write(RegisterAddress::GpioConfig, 0b_0000_0000)
            .await?;
        self.register_write(RegisterAddress::GpioDriveConfig, 0b_0000_0000)
            .await
    }

    pub(crate) async fn configure_oversampling(
        &mut self,
        ratio: Oversampling,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.write_oversampling_config(ratio).await
    }

    pub(crate) async fn configure_auto_sequence_mode(
        &mut self,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.write_data_config(DataConfig::NormalDataAddChannelID)
            .await?;
        self.write_sequence_config(SequenceConfig::StoppedAuto)
            .await?;
        self.register_write(RegisterAddress::AutoSequenceChannelSelect, 0b_1111_1111)
            .await
    }

    pub(crate) async fn read_system_status(
        &mut self,
    ) -> Result<SystemStatusFlags, <I2C as ErrorType>::Error> {
        let bits = self.register_read(RegisterAddress::SystemStatus).await?;
        Ok(SystemStatusFlags::from_bits_retain(bits))
    }

    pub(crate) async fn read_general_config(
        &mut self,
    ) -> Result<GeneralConfigFlags, <I2C as ErrorType>::Error> {
        let bits = self.register_read(RegisterAddress::GeneralConfig).await?;
        Ok(GeneralConfigFlags::from_bits_retain(bits))
    }

    pub(crate) async fn write_general_config(
        &mut self,
        config: GeneralConfigFlags,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::DataConfig, config.bits())
            .await
    }

    pub(crate) async fn write_data_config(
        &mut self,
        config: DataConfig,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::DataConfig, config.value())
            .await
    }

    pub(crate) async fn write_oversampling_config(
        &mut self,
        config: Oversampling,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::OsrConfig, config.value())
            .await
    }

    pub(crate) async fn write_sequence_config(
        &mut self,
        config: SequenceConfig,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.register_write(RegisterAddress::SequenceConfig, config.value())
            .await
    }

    async fn register_read(&mut self, r: RegisterAddress) -> Result<u8, <I2C as ErrorType>::Error> {
        let mut incoming = [0_u8; 1];
        let res = self
            .i2c
            .write_read(
                self.address,
                &[OpCode::SingleRegisterRead.value(), r.value()],
                &mut incoming,
            )
            .await;
        match res {
            Ok(()) => Ok(incoming[0]),
            Err(err) => Err(err),
        }
    }

    async fn register_write(
        &mut self,
        r: RegisterAddress,
        val: u8,
    ) -> Result<(), <I2C as ErrorType>::Error> {
        self.i2c
            .write(
                self.address,
                &[OpCode::SingleRegisterWrite.value(), r.value(), val],
            )
            .await
    }

    pub(crate) async fn data_read(&mut self) -> Result<[u16; 8], <I2C as ErrorType>::Error> {
        let mut data_buffer = [0_u8; (8 * 3)];
        self.i2c.read(self.address, &mut data_buffer).await?;

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
