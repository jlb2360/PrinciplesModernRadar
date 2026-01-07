use crate::core::utils::factorial;
pub fn m_n_detection(
    p_d_single: f64,
    p_fa_single: f64,
    m: u128,
    n: u128
) -> (f64, f64){

    let mut p_d_new = 0.0;
    for k in m..=n {
       p_d_new += (
                factorial(n) / (factorial(k) * factorial(n-k))
            ) as f64 *
            p_d_single.powi(k as i32) *
            (1.0 - p_d_single).powi((n-k) as i32);
    }

    let mut p_fa_new = 0.0;
    for k in m..=n {
       p_fa_new += (
                factorial(n) / (factorial(k) * factorial(n-k))
            ) as f64 *
            p_fa_single.powi(k as i32) *
            (1.0 - p_fa_single).powi((n-k) as i32);
    }

    (p_d_new, p_fa_new)
}
