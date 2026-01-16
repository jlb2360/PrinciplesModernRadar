use radar_toolbox::core::constants::C;
use radar_toolbox::systems::physics::simple::doppler_frequency;
pub fn solve(){
    println!("---- Solving problem 1 for chapter 8 ----");

    let vel = 100.0;
    let fc = 1E9;

    let fd_full = (1.0 + vel/C)/(1.0 - vel/C) * fc - fc;
    let fd_approx = doppler_frequency(fc, vel);

    println!("doppler frequency {:.2}", fd_approx);

    let error = 1.0 - fd_approx/fd_full;

    println!("The error from the approximation is {:.2e}", error);



}
