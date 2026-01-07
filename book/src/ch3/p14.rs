use radar_toolbox::targets::detection;
pub fn solve(){
    println!("---- Solving problem 14 for chapter 3 ----");

    let p_d_single = 0.75;
    let p_fa_single = 5E-3;

    let (p_d_2_3, p_fa_2_3) = detection::m_n_detection(p_d_single, p_fa_single, 2, 3);
    let (p_d_2_4, p_fa_2_4) = detection::m_n_detection(p_d_single, p_fa_single, 2, 4);


    println!(
    "Probability of detection and Probability of false acquisition for m = 2 and n = 3 ({:.2}, {:.2e})",
    p_d_2_3, p_fa_2_3);

    println!(
    "Probability of detection and Probability of false acquisition for m = 2 and n = 4 ({:.2}, {:.2e})",
    p_d_2_4, p_fa_2_4);


}
