use radar_toolbox::targets::detection;

pub fn solve(){
    println!("---- Solving problem 16 for chapter 3 ----");

    let p_d_single = 0.9;
    let p_fa_single = 0.01;

    let (p_d_3_5, p_fa_3_5) = detection::m_n_detection(p_d_single, p_fa_single, 3, 5);

    let az_n = (10.0_f64 / 2.7_f64).ceil();
    let el_n = (10.0_f64 / 2.7_f64).ceil();
    let n_scans = az_n * el_n;
    let n_bins = 40E3/150.0 * 64.0;

    let single_fa_n = n_bins * n_scans * p_fa_single;
    let multi_fa_n = n_bins * n_scans * p_fa_3_5;

    println!(
    "Probability of detection and Number of false acquisition for single ({:.2}, {:.2})",
    p_d_single, single_fa_n);

    println!(
    "Probability of detection and Number of false acquisition for m = 3 and n = 5 ({:.2}, {:.2})",
    p_d_3_5, multi_fa_n);


}
