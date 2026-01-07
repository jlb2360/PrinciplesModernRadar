
pub fn linear_to_db(lin: f64) -> f64 {
    10.0 * lin.log10()
}

pub fn db_to_linear(db: f64) -> f64 {
    10.0f64.powf(db / 10.0)
}

pub fn factorial(n: u128) -> u128 {
    (1..=n).product()
}


