use radar_toolbox::targets::detection;
use radar_toolbox::targets::swerling;
use radar_toolbox::targets::target_traits::Pd;

pub fn solve(){
    println!("---- Solving problem 15 for chapter 3 ----");

    let p_d_single = 0.75;
    let p_fa_single = 5E-3;

    let (p_d_2_4, p_fa_2_4) = detection::m_n_detection(p_d_single, p_fa_single, 2, 4);


    let target = swerling::Sw1::new(0.0);

    let snr_single = target.snr(p_d_single, p_fa_single);
    let snr_multiple = target.snr(p_d_2_4, p_fa_2_4);

    let snr_increase = 10.0*snr_multiple.log10() - 10.0*snr_single.log10();

    println!("The necessary increase in snr is {:.2} dB", {snr_increase});

}
