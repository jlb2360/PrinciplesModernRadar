use radar_toolbox::targets::clutter::clutter_volume;
use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 2 for chapter 5 ----");

    let mut v = clutter_volume(3.0_f64.to_radians(), 3.0_f64.to_radians(), 10E3, C*10E-6/2.0);


    println!("The Volume of Clutter cell for 10 km {:.2} m^3", v);
    v = clutter_volume(3.0_f64.to_radians(), 3.0_f64.to_radians(), 50E3, C*10E-6/2.0);

    println!("The Volume of Clutter cell for 50 km {:.2} m^3", v);
}
