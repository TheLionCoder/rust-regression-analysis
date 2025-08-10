use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SalaryRecord {
    pub work_year: f64,
    pub salary_in_usd: f64,
}

#[derive(Debug)]
pub struct Data {
    pub work_years: ndarray::Array1<f64>,
    pub salaries_in_usd: ndarray::Array1<f64>,
}
pub fn load_data() -> Result<Data, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./assets/ds_salaries.csv")?;

    let records = reader
        .deserialize()
        .collect::<Result<Vec<SalaryRecord>, csv::Error>>()?;

    let mut work_years = Vec::with_capacity(records.len());
    let mut salaries_in_usd = Vec::with_capacity(records.len());

    for record in records {
        work_years.push(record.work_year);
        salaries_in_usd.push(record.salary_in_usd);
    }

    Ok(Data {
        work_years: ndarray::Array1::from(work_years),
        salaries_in_usd: ndarray::Array1::from(salaries_in_usd),
    })
}
