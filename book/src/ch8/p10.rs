use radar_toolbox::systems::physics::simple::doppler_frequency;
pub fn solve(){
    println!("---- Solving problem 10 for chapter 8 ----");

    let vel_res = 1.0;
    let fc = 6E9;

    let fd_res = doppler_frequency(fc, vel_res);

    println!("doppler resolution {:.2} Hz", fd_res);

    let prf = 1000.0;

    let m = prf/fd_res;

    println!("Number of needed pulses is {:.2} #", m);



}
