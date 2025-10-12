//! Constants for the black hole reflection simulation.

/// Threshold for the radial factor to determine if a black hole has formed.
pub const BH_RADIAL_FACTOR_THRESHOLD: f64 = 0.02;

/// Minimum Courant number for the time step.
pub const MIN_COURANT_NUMBER: f64 = 0.02;

/// Maximum Courant number for the time step.
pub const MAX_COURANT_NUMBER: f64 = 0.95;

/// Epsilon for floating point comparisons.
pub const EPS: f64 = 1e-12;

/// Dissipation factor. Must be less than 1 to maintain 4th order accuracy.
pub const DISSIPATION_FACTOR: f64 = 0.2;

/// The steepness of the initial wave profile.
pub const INITIAL_WAVE_STEEPNESS: f64 = 64.0;

/// Steps per progress update log message.
pub const STEPS_PER_PROGRESS_UPDATE: i32 = 100;
