use radar_toolbox::signals::propagation::multipath_attenuation_factor;
pub fn solve(){
    println!("---- Solving problem 15 for chapter 4 ----");

    let pow = (
        multipath_attenuation_factor(
            0.0, 
            0.0, 
            radar_toolbox::Complex { re: 1.0, im: 0.0}) *
        multipath_attenuation_factor(
            0.0,
            0.0, 
            radar_toolbox::Complex { re: 1.0, im: 0.0 })
    ).abs().powi(2) * 1.0;

    println!("Power from signal {:.2} dB", 10.0*pow.log10());

}
