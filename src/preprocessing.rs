use rayon::prelude::*;

pub fn normalize_feature_inplace(feature: &mut ndarray::Array1<f64>) -> Result<(), &'static str> {
    let mean = feature
    .mean()
    .ok_or("Cannot calculate man for the feature.")?;

    let std_dev = feature.std(1.0);

    if std_dev == 0.0 {
        return Ok(());
    }

    feature.iter_mut().par_bridge().for_each(|x| *x = (*x - mean) / std_dev);

    Ok(())
}