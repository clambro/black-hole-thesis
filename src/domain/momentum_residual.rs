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
    let [m_n_minus_3, m_n_minus_2, m_n_minus_1, m_n] = &mass_history.masses;
    let [t_n_minus_3, t_n_minus_2, t_n_minus_1, t_n] = &mass_history.times;

    // 4th-order backward difference formula for variable time steps
    // Using polynomial interpolation through 4 points and differentiating
    let h1 = t_n - t_n_minus_1;
    let h2 = t_n_minus_1 - t_n_minus_2;
    let h3 = t_n_minus_2 - t_n_minus_3;

    // Calculate the coefficients for 4th-order BDF with variable steps
    // These are derived from polynomial interpolation and differentiation
    let h1_h2 = h1 + h2;
    let h1_h2_h3 = h1_h2 + h3;

    let alpha0 = (h1_h2_h3 + h1_h2 + h1) / (h1 * h1_h2 * h1_h2_h3);
    let alpha1 = -(h1_h2_h3 + h1_h2) / (h1 * h1_h2 * h2);
    let alpha2 = h1_h2_h3 / (h1_h2 * h2 * h3);
    let alpha3 = -h1_h2 / (h1_h2_h3 * h2 * h3);

    return alpha0 * m_n + alpha1 * m_n_minus_1 + alpha2 * m_n_minus_2 + alpha3 * m_n_minus_3;
}
