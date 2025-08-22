use data_loader::load_data;
use ndarray::{array, stack};
use regression::{linear::linear_regression, polynomial::generate_polynomial_features};

use crate::{
    preprocessing::normalize_feature_inplace,
    regression::multiple_linear::multiple_linear_regression,
};

mod data_loader;
mod preprocessing;
mod regression;
mod regression_error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set a global default subscriber.");
    let mut data = load_data()?;

    let degree = 2;

    normalize_feature_inplace(&mut data.work_years)?;
    normalize_feature_inplace(&mut data.remote_ratios)?;

    let (beta0, beta1) = linear_regression(&data.experience_levels, &data.salaries_in_usd).unwrap();

    tracing::info!("Intercept (ß0): {:.2}", beta0);
    tracing::info!("Slope (ß1): {:.2}", beta1);

    // Predict salaries for specific work years
    let test_levels = array![1.0, 2.0, 3.0];
    let predicted_salaries = beta0 + beta1 * &test_levels;
    tracing::info!("Predicted salaries: {:?}", predicted_salaries);

    let features = stack(
        ndarray::Axis(1),
        &[data.work_years.view(), data.remote_ratios.view()],
    )?;

    let beta = multiple_linear_regression(&features, &data.salaries_in_usd)?;

    tracing::info!("Coefficients: {:?}", beta);

    // Predict new data
    let test_data = array![[1.0, 5.0, 0.5], [1.0, 10.0, 0.2], [1.0, 15.0, 0.8]];

    let y_predict = test_data.dot(&beta);
    tracing::info!("Predictes Salaries: {:?}", y_predict);

    let matrix_poly = generate_polynomial_features(&data.experience_levels, degree);

    let beta_poly = multiple_linear_regression(
        &matrix_poly.slice(ndarray::s![.., 1..]).to_owned(),
        &data.salaries_in_usd,
    )?;
    tracing::info!("Poly Coefficients: {:?}", beta_poly);

    // Predict new data
    let test_matrix_poly = generate_polynomial_features(&test_levels, degree);
    let y_pred = test_matrix_poly.dot(&beta_poly);

    tracing::info!("Predicted Salaries: {:?}", y_pred);

    Ok(())
}
