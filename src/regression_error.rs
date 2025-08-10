#[derive(Debug, PartialEq)]
pub enum RegressionError {
    EmptyImput,
    MismatchedLenght,
    NoVariance,
}
