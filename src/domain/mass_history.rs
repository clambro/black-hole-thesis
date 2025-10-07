use crate::domain::field_vector::FieldVector;

/// Stores exactly 5 mass values with their corresponding times for 4th-order accurate time derivative calculation.
#[derive(Debug, Clone)]
pub struct MassHistory {
    pub masses: [FieldVector; 5],
    pub times: [f64; 5],
}

impl MassHistory {
    /// Create a new MassHistory with 5 identical mass values at the same time
    pub fn new(initial_mass: FieldVector, initial_time: f64) -> Self {
        Self {
            masses: [
                initial_mass.clone(),
                initial_mass.clone(),
                initial_mass.clone(),
                initial_mass.clone(),
                initial_mass,
            ],
            times: [initial_time; 5],
        }
    }

    /// Add a new mass value to the history, returning a new MassHistory
    pub fn add_mass(&self, mass: FieldVector, time: f64) -> Self {
        Self {
            masses: [
                self.masses[1].clone(),
                self.masses[2].clone(),
                self.masses[3].clone(),
                self.masses[4].clone(),
                mass,
            ],
            times: [
                self.times[1],
                self.times[2],
                self.times[3],
                self.times[4],
                time,
            ],
        }
    }
}
