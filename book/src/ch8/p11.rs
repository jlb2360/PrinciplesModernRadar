use std::f64::consts::PI;

use radar_toolbox::systems::physics::simple::doppler_frequency;
use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 11 for chapter 8 ----");

    let vel = 50.0;
    let fc = 2E9;
    let wavelength = C/fc;

    let fd = doppler_frequency(fc, vel);

    let prf = 2E3;
    let pri = 1.0/prf;

    let ph = 2.0 * PI * fd * pri;

    let dr_wavelength = vel * pri / wavelength;


    println!("The phase change per pulse is {:.2} radians", ph);
    println!("The number of wavelengths the target moves per pulse is {:.2}", dr_wavelength);
    println!("The range change is {:.3} m", dr_wavelength * wavelength);



}
