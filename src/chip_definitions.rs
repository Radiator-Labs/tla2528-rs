use bitflags::bitflags;

#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum OpCode {
    SingleRegisterRead = 0b_0001_0000,
    SingleRegisterWrite = 0b_0000_1000,
    SetBit = 0b_0001_1000,
    ClearBit = 0b_0010_0000,
    ReadContinuousRegisters = 0b_0011_0000,
    WriteContinuousRegisters = 0b_0010_1000,
}
impl OpCode {
    pub(crate) fn value(self) -> u8 {
        self as u8
    }
}

#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum RegisterAddress {
    SystemStatus = 0x00,
    GeneralConfig = 0x01,
    DataConfig = 0x02,
    OsrConfig = 0x03,
    OpModeConfig = 0x04,
    PinConfig = 0x05,
    GpioConfig = 0x07,
    GpioDriveConfig = 0x09,
    GpOutValue = 0x0B,
    GpInValue = 0x0D,
    SequenceConfig = 0x10,
    ChannelSelect = 0x11,
    AutoSequenceChannelSelect = 0x12,
}
impl RegisterAddress {
    pub(crate) fn value(self) -> u8 {
        self as u8
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct SystemStatusFlags: u8 {
        const _RESERVED_07 = 0b_1000_0000;
        const SEQUENCER_IN_PROGRESS = 0b_0100_0000;
        const I2C_HIGH_SPEED = 0b_0010_0000;
        const _RESERVED_04 = 0b_0001_0000;
        const OSR_COMPLETE = 0b_0000_1000;
        const CRC_ERROR = 0b_0000_0100;
        const _RESERVED_01 = 0b_0000_0010;
        const BROWNOUT_RESET = 0b_0000_0001;
        const NULL = 0b_0000_0000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct GeneralConfigFlags: u8 {
        const _RESERVED_07 = 0b_1000_0000;
        const _RESERVED_06 = 0b_0100_0000;
        const _RESERVED_05 = 0b_0010_0000;
        const _RESERVED_04 = 0b_0001_0000;
        const INITIATE_CONVERSION = 0b_0000_1000;
        const OVERRIDE_OTHER_SETTINGS_TO_ALL_CHANNELS_AS_ANALOG_INPUTS = 0b_0000_0100;
        const CALIBRATE_ADC_OFFSET = 0b_0000_0010;
        const RESET = 0b_0000_0001;
        const NULL = 0b_0000_0000;
    }
}

#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum DataConfig {
    NormalDataNoChannelID = 0b_0000_0000,
    NormalDataAddChannelID = 0b_0001_0000,
    FixedDataNoChannelID = 0b_1000_0000,
    FixedDataAddChannelID = 0b_1001_0000,
}
impl DataConfig {
    pub(crate) fn value(self) -> u8 {
        self as u8
    }
}

#[allow(dead_code)]
#[repr(u8)]
#[non_exhaustive]
pub enum Oversampling {
    Ratio0 = 0b_0000_0000,
    Ratio2 = 0b_0000_0001,
    Ratio4 = 0b_0000_0010,
    Ratio8 = 0b_0000_0011,
    Ratio16 = 0b_0000_0100,
    Ratio32 = 0b_0000_0101,
    Ratio64 = 0b_0000_0110,
    Ratio128 = 0b_0000_0111,
}
impl Oversampling {
    pub(crate) fn value(self) -> u8 {
        self as u8
    }
}

#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum SequenceConfig {
    Manual = 0b_0000_0000,
    StoppedAuto = 0b_0000_0001,
    StartedAuto = 0b_0001_0001,
}
impl SequenceConfig {
    pub(crate) fn value(self) -> u8 {
        self as u8
    }
}
