use bitflags::bitflags;

#[allow(
    dead_code,
    reason = "Defines all options in the interface, even those that are unused."
)]
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

#[allow(
    dead_code,
    reason = "Defines all options in the interface, even those that are unused."
)]
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

#[allow(
    dead_code,
    reason = "Defines all options in the interface, even those that are unused."
)]
#[derive(Debug)]
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

#[allow(
    dead_code,
    reason = "Defines all options in the interface, even those that are unused."
)]
#[derive(Debug)]
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

#[allow(
    dead_code,
    reason = "Defines all options in the interface, even those that are unused."
)]
#[allow(
    non_camel_case_types,
    reason = "Underscores create clarity in enum values."
)]
#[derive(Debug)]
#[repr(u8)]
#[non_exhaustive]
pub enum SamplingRate {
    HighSpeedOscillator_1_000_xxx_Sps = 0b_0000_0000,
    HighSpeedOscillator_666_7xx_Sps = 0b_0000_0001,
    HighSpeedOscillator_500_xxx_Sps = 0b_0000_0010,
    HighSpeedOscillator_333_3xx_Sps = 0b_0000_0011,
    HighSpeedOscillator_250_xxx_Sps = 0b_0000_0100,
    HighSpeedOscillator_166_7xx_Sps = 0b_0000_0101,
    HighSpeedOscillator_125_xxx_Sps = 0b_0000_0110,
    HighSpeedOscillator_83_xxx_Sps = 0b_0000_0111,
    HighSpeedOscillator_62_5xx_Sps = 0b_0000_1000,
    HighSpeedOscillator_41_7xx_Sps = 0b_0000_1001,
    HighSpeedOscillator_31_3xx_Sps = 0b_0000_1010,
    HighSpeedOscillator_20_8xx_Sps = 0b_0000_1011,
    HighSpeedOscillator_15_6xx_Sps = 0b_0000_1100,
    HighSpeedOscillator_10_4xx_Sps = 0b_0000_1101,
    HighSpeedOscillator_7_8xx_Sps = 0b_0000_1110,
    HighSpeedOscillator_5_2xx_Sps = 0b_0000_1111,
    LowSpeedOscillator_31_25x_Sps = 0b_0001_0000,
    LowSpeedOscillator_20_83x_Sps = 0b_0001_0001,
    LowSpeedOscillator_15_63x_Sps = 0b_0001_0010,
    LowSpeedOscillator_10_42x_Sps = 0b_0001_0011,
    LowSpeedOscillator_7_81x_Sps = 0b_0001_0100,
    LowSpeedOscillator_5_21x_Sps = 0b_0001_0101,
    LowSpeedOscillator_3_91x_Sps = 0b_0001_0110,
    LowSpeedOscillator_2_60x_Sps = 0b_0001_0111,
    LowSpeedOscillator_1_95x_Sps = 0b_0001_1000,
    LowSpeedOscillator_1_30x_Sps = 0b_0001_1001,
    LowSpeedOscillator_0_98x_Sps = 0b_0001_1010,
    LowSpeedOscillator_0_65x_Sps = 0b_0001_1011,
    LowSpeedOscillator_0_49x_Sps = 0b_0001_1100,
    LowSpeedOscillator_0_33x_Sps = 0b_0001_1101,
    LowSpeedOscillator_0_24x_Sps = 0b_0001_1110,
    LowSpeedOscillator_0_16x_Sps = 0b_0001_1111,
}
impl SamplingRate {
    pub(crate) fn value(self) -> u8 {
        self as u8
    }
}

#[allow(
    dead_code,
    reason = "Defines all options in the interface, even those that are unused."
)]
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
