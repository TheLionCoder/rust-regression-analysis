use data_loader::load_data;
use ndarray::array;
use regression::linear::linear_regression;

mod data_loader;
mod regression;
mod regression_error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set a global default subscriber.");
    let data = load_data()?;
    let start_year = data.work_years.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let centered_years: ndarray::Array1<f64> = data.work_years.mapv(|year| year - start_year);

    tracing::info!("Data centered around starting year: {}", start_year);

    let (beta0, beta1) = linear_regression(&centered_years, &data.salaries_in_usd).unwrap();

    tracing::info!("Intercept (ß0): {:.2}", beta0);
    tracing::info!("Slope (ß1): {:.2}", beta1);

    // Predict salaries for specific work years
    let test_years = array![5.0, 10.0, 15.0];
    let predicted_salaries = beta0 + beta1 * &test_years;
    tracing::info!("Predicted salaries: {:?}", predicted_salaries);

    Ok(())
}
