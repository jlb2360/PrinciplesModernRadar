use std::f64::consts::PI;

use radar_toolbox::core::constants::C;
use radar_toolbox::antennas::simple::max_directivity;
pub fn solve(){
    println!("---- Solving problem 5 for chapter 9 ----");
    
    let lam = C/16E9;
    let rad = 2.0 * 0.3048 / 2.0;

    let area = PI * rad * rad;

    let dir_max = max_directivity(1.0, area, lam);

    let dir_e = (-(15.0_f64.to_radians().powi(2))).exp() * dir_max;

    let g_db = 10.0*dir_e.log10() - 2.0; // rf losses

    println!("Max directivity is {:.2} dB", 10.0*dir_max.log10());
    println!("Real directivity is {:.2} dB", 10.0*dir_e.log10());
    println!("Antenna Gain is {:.2} dB", g_db);

}
