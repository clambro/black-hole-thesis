//! Constants for the black hole reflection simulation.

/// Threshold for the radial factor to start tracking black hole formation.
pub const BH_RADIAL_FACTOR_TRACKING: f64 = 0.1;

/// Threshold for the radial factor to determine that a black hole has formed.
pub const BH_RADIAL_FACTOR_THRESHOLD: f64 = 0.01;

/// Courant number for calculating the time step. Must be less than 1.
/// I'm shocked at how high this is. No adaptivity is needed.
pub const COURANT_NUMBER: f64 = 0.95;

/// Factor to slow down the time step by when we start tracking black hole formation.
pub const BH_SLOWDOWN_FACTOR: f64 = 0.25;

/// Epsilon for floating point comparisons.
pub const EPS: f64 = 1e-12;

/// Dissipation factor. Must be less than 1 to maintain 4th order accuracy.
pub const DISSIPATION_FACTOR: f64 = 0.01;

/// The steepness of the initial wave profile.
pub const INITIAL_WAVE_STEEPNESS: f64 = 64.0;

/// Steps per progress update log message.
pub const STEPS_PER_PROGRESS_UPDATE: i32 = 100;
