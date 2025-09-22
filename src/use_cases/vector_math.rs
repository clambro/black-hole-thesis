use rayon::prelude::*;

/// Add two vectors.
pub fn vec_add(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    a.par_iter().zip(b.par_iter()).map(|(x, y)| x + y).collect()
}

/// Multiply a vector by a scalar.
pub fn vec_scalar_mul(scalar: f64, vec: &Vec<f64>) -> Vec<f64> {
    vec.par_iter().map(|x| scalar * x).collect()
}
