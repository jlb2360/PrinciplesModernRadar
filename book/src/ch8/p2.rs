use radar_toolbox::core::constants::C;
use radar_toolbox::systems::physics::simple::doppler_frequency;
use radar_toolbox::core::types::Vec3;
pub fn solve(){
    println!("---- Solving problem 2 for chapter 8 ----");


    let p1 = Vec3::new(100.0, 0.0, 0.0);
    let p2 = Vec3::new(-100.0/2.0_f64.sqrt(), -100.0/2.0_f64.sqrt(), 0.0);

    let vel_r = p2 - p1;
    let u = Vec3::new(1.0/2.0_f64.sqrt(), 1.0/2.0_f64.sqrt(), 0.0);

    println!("U ({}, {}, {})", u.x, u.y, u.z);

    let vr = vel_r.dot(u);

    println!("The radial velocity is {:.2} m/s", vr);

    let fc = 10E9;
    let fd = -doppler_frequency(fc, vr); // negative because closing is positive

    println!("The doppler frequency seen is {:.2} Hz", fd);


}
