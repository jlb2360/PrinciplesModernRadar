pub mod core;
pub mod systems;
pub mod signals;
pub mod antennas;
pub mod targets;

// 2. Re-export the most common types for easy access
// User can write `use radar_lib::Complex` instead of `use radar_lib::core::types::Complex`
pub use crate::core::types::Complex;
pub use crate::core::constants::*;
