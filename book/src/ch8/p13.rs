use std::f64::consts::PI;

use radar_toolbox::systems::physics::simple::doppler_frequency;
use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 13 for chapter 8 ----");

    let vel = 150.0;
    let fc = 10E9;
    let wavelength = C/fc;

    let psi = 30.0_f64;
    let th_3 = 4.0_f64;

    let b_mlc = 4.0 * vel / wavelength * (th_3.to_radians()/2.0).sin() * psi.to_radians().sin();

    println!("The doppler shift from the ground clutter is {:.2} Hz", b_mlc);

}
