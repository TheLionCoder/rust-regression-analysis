use ndarray::prelude::*;

use crate::regression_error;

pub fn linear_regression(
    x: &Array1<f64>,
    y: &Array1<f64>,
) -> Result<(f64, f64), regression_error::RegressionError> {
    if x.is_empty() || y.is_empty() {
        return Err(regression_error::RegressionError::EmptyImput);
    }

    if x.len() != y.len() {
        return Err(regression_error::RegressionError::MismatchedLenght);
    }

    let x_mean = x.mean().unwrap();
    let y_mean = y.mean().unwrap();

    let denominator = x.iter().map(|&value| (value - x_mean).powi(2)).sum::<f64>();

    if denominator == 0.0 {
        return Err(regression_error::RegressionError::NoVariance);
    }

    let numerator = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - x_mean) * (yi - y_mean))
        .sum::<f64>();

    let beta1 = numerator / denominator;
    let beta0 = y_mean - beta1 * x_mean;

    Ok((beta0, beta1))
}
