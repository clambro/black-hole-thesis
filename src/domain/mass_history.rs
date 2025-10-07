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

    pub fn calculate_mass_time_derivative(&self) -> FieldVector {
        let [m_0, m_1, m_2, m_3, m_4] = &self.masses;
        let [t_0, t_1, t_2, t_3, t_4] = &self.times;

        // 4th-order backward difference formula for variable time steps
        // Using 5 points: t_0 < t_1 < t_2 < t_3 < t_4
        let h1 = t_4 - t_3;
        let h2 = t_3 - t_2;
        let h3 = t_2 - t_1;
        let h4 = t_1 - t_0;

        let w0 = h1 * (h1 + h2) * (h1 + h2 + h3)
            / (h4 * (h3 + h4) * (h2 + h3 + h4) * (h1 + h2 + h3 + h4));
        let w1 = -h1 * (h1 + h2) * (h1 + h2 + h3 + h4) / (h3 * h4 * (h2 + h3) * (h1 + h2 + h3));
        let w2 = h1 * (h1 + h2 + h3) * (h1 + h2 + h3 + h4) / (h2 * h3 * (h1 + h2) * (h3 + h4));
        let w3 = -(h1 + h2) * (h1 + h2 + h3) * (h1 + h2 + h3 + h4)
            / (h1 * h2 * (h2 + h3) * (h2 + h3 + h4));
        let w4 = 1.0 / h1 + 1.0 / (h1 + h2) + 1.0 / (h1 + h2 + h3) + 1.0 / (h1 + h2 + h3 + h4);

        return w0 * m_0 + w1 * m_1 + w2 * m_2 + w3 * m_3 + w4 * m_4;
    }
}
