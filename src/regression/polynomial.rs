pub fn generate_polynomial_features(
    x: &ndarray::Array1<f64>,
    degree: usize,
) -> ndarray::Array2<f64> {
    let n_samples = x.len();

    // The number for output features is degree + 1 (for powers from 0 to `degree`let n_features = n_samples + 1;
    let n_features = degree + 1;

    ndarray::Array2::from_shape_fn((n_samples, n_features), |(i, j)| x[i].powi(j as i32))
}
