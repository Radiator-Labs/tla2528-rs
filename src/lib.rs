#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![deny(clippy::alloc_instead_of_core)]
// #![allow(clippy::allow_attributes)] // coming in 1.70.0
#![deny(clippy::allow_attributes_without_reason)]
#![allow(
    clippy::arithmetic_side_effects,
    reason = "Kelvin Embedded style guide: Arithmetic side effects commonly used in embedded systems programming."
)]
#![allow(
    clippy::as_conversions,
    reason = "clippy::as_conversions explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::as_underscore)]
#![deny(clippy::assertions_on_constants)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::create_dir)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::decimal_literal_representation)]
#![deny(clippy::default_numeric_fallback)]
#![deny(clippy::default_union_representation)]
#![deny(clippy::deref_by_slicing)]
#![deny(clippy::disallowed_script_idents)]
#![allow(
    clippy::duplicated_attributes,
    reason = "https://github.com/rust-lang/rust-clippy/issues/13500"
)]
#![deny(clippy::else_if_without_else)]
#![deny(clippy::empty_drop)]
#![deny(clippy::empty_structs_with_brackets)]
#![allow(
    clippy::exhaustive_enums,
    reason = "Kelvin Embedded style guide: allowing public enumerants is consistent with a functional style use of enumerations as pure data."
)]
#![allow(
    clippy::exhaustive_structs,
    reason = "Kelvin Embedded style guide: allowing public field access is consistent with a functional style use of data structures as pure data."
)]
#![deny(clippy::exit)]
#![deny(clippy::expect_used)]
#![deny(clippy::filetype_is_file)]
#![deny(clippy::float_arithmetic)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::fn_to_numeric_cast_any)]
#![deny(clippy::format_push_string)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::impl_trait_in_params)]
#![allow(
    clippy::implicit_return,
    reason = "Kelvin Embedded Style Guide: Implicit returns are an idiomatic approach that improves code readability."
)]
#![allow(
    clippy::indexing_slicing,
    reason = "Required by defmt. TODO fix this in defmt library"
)]
#![deny(clippy::inline_asm_x86_att_syntax)]
#![allow(
    clippy::inline_asm_x86_intel_syntax,
    reason = "clippy::inline_asm_x86_intel_syntax explanation lost in history. TODO remove allow and find reason."
)]
#![allow(
    clippy::integer_division,
    reason = "Kelvin Embedded Style Guide: Integer division is a normally used operation in embedded systems programming."
)]
#![deny(clippy::large_include_file)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::let_underscore_untyped)]
#![deny(clippy::lossy_float_literal)]
#![deny(clippy::map_err_ignore)]
#![deny(clippy::mem_forget)]
// #![deny(clippy::missing_assert_message)] // coming in 1.70.0
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "clippy::missing_docs_in_private_items explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::missing_enforced_import_renames)]
#![allow(
    clippy::missing_inline_in_public_items,
    reason = "clippy::missing_inline_in_public_items explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::missing_trait_methods)]
#![deny(clippy::mixed_read_write_in_expression)]
#![deny(clippy::mod_module_files)]
#![allow(
    clippy::modulo_arithmetic,
    reason = "clippy::modulo_arithmetic explanation lost in history. TODO remove allow and find reason."
)]
#![allow(
    clippy::multiple_crate_versions,
    reason = "clippy::multiple_crate_versions explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::multiple_inherent_impl)]
#![deny(clippy::multiple_unsafe_ops_per_block)]
#![deny(clippy::mutex_atomic)]
#![deny(clippy::non_ascii_literal)]
#![deny(clippy::panic)]
#![deny(clippy::panic_in_result_fn)]
#![deny(clippy::partial_pub_fields)]
#![deny(clippy::pattern_type_mismatch)]
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]
#![deny(clippy::pub_use)]
#![allow(
    clippy::question_mark_used,
    reason = "Allowed by Kelvin Style Guide. This is idiomatic Rust."
)]
#![deny(clippy::rc_buffer)]
#![deny(clippy::rc_mutex)]
#![deny(clippy::redundant_feature_names)]
// #![deny(clippy::ref_patterns)] // coming in 1.70.0
#![deny(clippy::rest_pat_in_fully_bound_structs)]
#![allow(
    clippy::same_name_method,
    reason = "TODO reconsider. Due to bitflags! macro"
)]
#![deny(clippy::self_named_module_files)]
#![deny(clippy::semicolon_outside_block)]
#![allow(
    clippy::separated_literal_suffix,
    reason = "clippy::separated_literal_suffix explanation lost in history. TODO remove allow and find reason."
)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_unrelated)]
#![allow(
    clippy::single_char_lifetime_names,
    reason = "Allowed by Kelvin style guide. Single character lifetime names are commonly used in idiomatic Rust."
)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![deny(clippy::str_to_string)]
#![deny(clippy::string_add)]
#![deny(clippy::string_slice)]
#![deny(clippy::string_to_string)]
#![deny(clippy::suspicious_xor_used_as_pow)]
//#![deny(clippy::tests_outside_test_module)] // coming in 1.70.0
#![deny(clippy::todo)]
#![deny(clippy::try_err)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unnecessary_safety_comment)]
#![deny(clippy::unnecessary_safety_doc)]
#![deny(clippy::unnecessary_self_imports)]
#![deny(clippy::unneeded_field_pattern)]
#![deny(clippy::unreachable)]
#![deny(clippy::unseparated_literal_suffix)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::use_debug)]
#![deny(clippy::verbose_file_reads)]
#![deny(clippy::wildcard_enum_match_arm)]

pub mod channel;
pub mod chip_definitions;
mod chip_interface;
pub mod error;

use crate::{
    channel::Channel,
    chip_definitions::{
        GeneralConfigFlags, Oversampling, SamplingRate, SequenceConfig, SystemStatusFlags,
    },
    chip_interface::ChipInterface,
    error::Error,
};
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub struct Tla2528<I2C> {
    chip: ChipInterface<I2C>,
}

impl<I2C> Tla2528<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Error<I2C::Error>>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Tla2528 {
            chip: ChipInterface::new(i2c, address),
        }
    }
    /// # Errors
    /// Passes on I2C errors found in `single_register_read()`
    ///
    /// Passes out I2C communication errors.
    pub fn get_system_status(&mut self) -> Result<SystemStatusFlags, Error<I2C::Error>> {
        self.chip.read_system_status()
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn calibrate(&mut self) -> Result<(), Error<I2C::Error>> {
        self.chip
            .write_general_config(GeneralConfigFlags::CALIBRATE_ADC_OFFSET)?;

        while self
            .chip
            .read_general_config()?
            .contains(GeneralConfigFlags::CALIBRATE_ADC_OFFSET)
        {
            // Intentionally empty
        }
        Ok(())
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn set_oversampling_ratio(&mut self, ratio: Oversampling) -> Result<(), Error<I2C::Error>> {
        self.chip.configure_oversampling(ratio)
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn set_sampling_rate(&mut self, rate: SamplingRate) -> Result<(), Error<I2C::Error>> {
        self.chip.configure_sampling_rate(rate)
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn prepare_for_auto_sequence_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.chip.configure_all_pins_as_analog_inputs()?;
        self.chip.configure_auto_sequence_mode()
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn prepare_for_manual_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.chip.configure_all_pins_as_analog_inputs()?;
        self.chip.configure_manual_mode()
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn acquire_data(&mut self) -> Result<[u16; 8], Error<I2C::Error>> {
        // Enable channel sequencing SEQ_START = 1
        self.chip
            .write_sequence_config(SequenceConfig::StartedAuto)?;

        // Host provides Conversion Start Frame on I2C Bus
        // goto here -> *
        // Host provides Conversion Read Frame on I2C Bus
        // continue; yes -> goto --^
        let data = self.chip.data_read()?;

        // channel sequencing SEQ_START = 0
        self.chip
            .write_sequence_config(SequenceConfig::StoppedAuto)?;

        Ok(data)
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn acquire_channel_data(&mut self, channel: Channel) -> Result<u16, Error<I2C::Error>> {
        self.chip.set_channel(channel)?;
        match self.chip.data_channel_read(channel) {
            Ok((data, _)) => Ok(data),
            Err(err) => Err(err),
        }
    }
}
