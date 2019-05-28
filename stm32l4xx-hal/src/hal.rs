// the goal is to replace
// lib.rs:pub use embedded_hal as hal;
// with our extended hal, containing
// - proven embedded_hal traits
// - unproven embedded_hal traits (behind "unproven" feature flag)
// - extra traits (e.g. for flash, behind "extra-traits" feature flag)

pub use embedded_hal::*;
#[cfg(feature = "extra-traits")]
pub use super::extra_traits::*;
