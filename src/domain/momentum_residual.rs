use crate::domain::config::Config;
use crate::domain::field_vector::FieldVector;
use crate::domain::mass_history::MassHistory;
use crate::domain::state::State;

pub fn calculate_momentum_residual(state: &State, config: &Config) -> f64 {
    let mass_time_derivative = calculate_mass_time_derivative(&state.mass_history);
    let expected_momentum_flux = config.grid.points.powi(2)
        * state.constraints.radial_factor.powi(2)
        / &state.constraints.lapse
        * &state.radial_gradient
        * &state.conj_momentum;

    let error = expected_momentum_flux - mass_time_derivative;
    return error.powi(2).sum().sqrt();
}

fn calculate_mass_time_derivative(mass_history: &MassHistory) -> FieldVector {
    let [m_0, m_1, m_2, m_3, m_4] = &mass_history.masses;
    let [t_0, t_1, t_2, t_3, t_4] = &mass_history.times;

    // 4th-order backward difference formula for variable time steps
    // Using 5 points: t_0 < t_1 < t_2 < t_3 < t_4
    let h1 = t_4 - t_3;
    let h2 = t_3 - t_2;
    let h3 = t_2 - t_1;
    let h4 = t_1 - t_0;

    let w0 =
        h1 * (h1 + h2) * (h1 + h2 + h3) / (h4 * (h3 + h4) * (h2 + h3 + h4) * (h1 + h2 + h3 + h4));
    let w1 = -h1 * (h1 + h2) * (h1 + h2 + h3 + h4) / (h3 * h4 * (h2 + h3) * (h1 + h2 + h3));
    let w2 = h1 * (h1 + h2 + h3) * (h1 + h2 + h3 + h4) / (h2 * h3 * (h1 + h2) * (h3 + h4));
    let w3 = -(h1 + h2) * (h1 + h2 + h3) * (h1 + h2 + h3 + h4)
        / (h1 * h2 * (h2 + h3) * (h2 + h3 + h4));
    let w4 = 1.0 / h1 + 1.0 / (h1 + h2) + 1.0 / (h1 + h2 + h3) + 1.0 / (h1 + h2 + h3 + h4);

    return w0 * m_0 + w1 * m_1 + w2 * m_2 + w3 * m_3 + w4 * m_4;
}
