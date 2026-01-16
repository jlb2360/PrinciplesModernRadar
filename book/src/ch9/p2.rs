use std::f64::consts::PI;

use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 2 for chapter 9 ----");
    let w = 3.5;

    let mut az = 15.0_f64.to_radians();
    let lam = C/10E9;

    let dphi = 2.0 * PI * w * az.sin() / lam;

    println!("The change of phase across the array in azimuth is {:.3} rads", dphi);

    az = 60.0_f64.to_radians();
    let dphi = 2.0 * PI * w * az.sin() / lam;

    println!("The change of phase across the array in azimuth is {:.3} rads", dphi);



}
