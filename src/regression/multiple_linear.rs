use ndarray_linalg::LeastSquaresSvd;

pub fn multiple_linear_regression(
    matrix: &ndarray::Array2<f64>,
    y: &ndarray::Array1<f64>,
) -> Result<ndarray::Array1<f64>, Box<dyn std::error::Error>> {
    // Add intercept term to matrix
    let ones = ndarray::Array2::ones((matrix.nrows(), 1));

    let matrix_with_intercept = ndarray::concatenate![ndarray::Axis(1), ones.view(), matrix.view()];

    let results = matrix_with_intercept.least_squares(y)?;

    Ok(results.solution)
}
