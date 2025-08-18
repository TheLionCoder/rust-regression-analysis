use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SalaryRecord {
    pub work_year: f64,
    pub salary_in_usd: f64,
    pub remote_ratio: f64,
}

#[derive(Debug)]
pub struct Data {
    pub work_years: ndarray::Array1<f64>,
    pub salaries_in_usd: ndarray::Array1<f64>,
    pub remote_ratios: ndarray::Array1<f64> 
}

pub fn load_data() -> Result<Data, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./assets/ds_salaries.csv")?;

    let records = reader
        .deserialize()
        .collect::<Result<Vec<SalaryRecord>, csv::Error>>()?;
    let n_rows = records.len();
    let mut work_years = Vec::with_capacity(n_rows);
    let mut salaries_in_usd = Vec::with_capacity(n_rows);
    let mut remote_ratios = Vec::with_capacity(n_rows);

    for record in records {
        work_years.push(record.work_year);
        salaries_in_usd.push(record.salary_in_usd);
        remote_ratios.push(record.remote_ratio);
    }


    Ok(Data {
        work_years: ndarray::Array1::from(work_years),
        salaries_in_usd: ndarray::Array1::from(salaries_in_usd),
        remote_ratios: ndarray::Array1::from(remote_ratios)
    })
}
