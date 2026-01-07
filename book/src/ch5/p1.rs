use radar_toolbox::targets::clutter::pulse_limited_gr;
use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 1 for chapter 5 ----");

    let mut gr = pulse_limited_gr(3.0_f64.to_radians(), 10E3, C*10E-6/2.0);


    println!("The grazing angle transition from beam to pulse limited for 10 km {:.2} degrees", gr.to_degrees());

    gr = pulse_limited_gr(3.0_f64.to_radians(), 50E3, C*10E-6/2.0);

    println!("The grazing angle transition from beam to pulse limited for 50 km {:.2} degrees", gr.to_degrees());

}
