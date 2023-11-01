#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![deny(clippy::alloc_instead_of_core)]
// #![allow(clippy::allow_attributes)] // coming in 1.70.0
#![deny(clippy::allow_attributes_without_reason)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::as_conversions)]
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
#![deny(clippy::else_if_without_else)]
#![deny(clippy::empty_drop)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::exhaustive_enums)]
#![deny(clippy::exhaustive_structs)]
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
#![allow(clippy::implicit_return)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::inline_asm_x86_att_syntax)]
#![allow(clippy::inline_asm_x86_intel_syntax)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::integer_division)]
#![deny(clippy::large_include_file)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::let_underscore_untyped)]
#![deny(clippy::lossy_float_literal)]
#![deny(clippy::map_err_ignore)]
#![deny(clippy::mem_forget)]
// #![deny(clippy::missing_assert_message)] // coming in 1.70.0
#![allow(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_enforced_import_renames)]
#![allow(clippy::missing_inline_in_public_items)]
#![deny(clippy::missing_trait_methods)]
#![deny(clippy::mixed_read_write_in_expression)]
#![deny(clippy::mod_module_files)]
#![allow(clippy::modulo_arithmetic)]
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
#![allow(clippy::question_mark_used)]
#![deny(clippy::rc_buffer)]
#![deny(clippy::rc_mutex)]
// #![deny(clippy::ref_patterns)] // coming in 1.70.0
#![deny(clippy::rest_pat_in_fully_bound_structs)]
#![allow(clippy::same_name_method)] // reconsider. Due to bitflags! macro
#![deny(clippy::self_named_module_files)]
#![deny(clippy::semicolon_outside_block)]
#![allow(clippy::separated_literal_suffix)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_unrelated)]
#![allow(clippy::single_char_lifetime_names)]
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

pub mod chip_definitions;
mod chip_interface;

use crate::chip_definitions::{
    GeneralConfigFlags, Oversampling, SequenceConfig, SystemStatusFlags,
};
use crate::chip_interface::{ChipInterface, Tla2528Error};
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub struct Tla2528<I2C> {
    chip: ChipInterface<I2C>,
}

impl<I2C> Tla2528<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: Into<Tla2528Error<I2C::Error>>,
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
    pub fn get_system_status(&mut self) -> Result<SystemStatusFlags, Tla2528Error<I2C::Error>> {
        self.chip.read_system_status()
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn calibrate(&mut self) -> Result<(), Tla2528Error<I2C::Error>> {
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
    pub fn set_oversampling_ratio(
        &mut self,
        ratio: Oversampling,
    ) -> Result<(), Tla2528Error<I2C::Error>> {
        self.chip.configure_oversampling(ratio)
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn prepare_for_auto_sequence_mode(&mut self) -> Result<(), Tla2528Error<I2C::Error>> {
        self.chip.configure_all_pins_as_analog_inputs()?;
        self.chip.configure_auto_sequence_mode()
    }

    /// # Errors
    ///
    /// Passes out I2C communication errors.
    pub fn acquire_data(&mut self) -> Result<[u16; 8], Tla2528Error<I2C::Error>> {
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
}
