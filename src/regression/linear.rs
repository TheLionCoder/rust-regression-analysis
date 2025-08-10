use ndarray::prelude::*;
use ndarray_stats::CorrelationExt;

pub fn linear_regression(x: Array1<f64>, y: Array1<f64>) -> (f64, f64) {
    let n = x.len() as f64;

    let x_mean = x.mean().unwrap();
    let y_mean = y.mean().unwrap();
}
