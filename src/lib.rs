#![no_std]

use defmt::Format;
pub mod bsp;
/// Generic Errors for this crate
#[derive(Debug, Clone, Copy, PartialEq, Format, Default)]
pub enum QuickError {
    ToBeDefined,
    #[default]
    FeatureNotImplemented,
    TimedOut {
        seconds: u32,
    },
    BadResponse,
    ArbitraryEnumValue {
        loving_rust_enums: bool,
    },
}

pub use bsp::*;
