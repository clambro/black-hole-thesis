/// Static configuration for the output.
pub struct OutputConfig {
    /// Time interval between output frames.
    pub dt: f64,
    /// Discretization level for output data.
    pub dx_level: u32,
    /// Whether to skip the state output.
    pub skip_state_output: bool,
}
