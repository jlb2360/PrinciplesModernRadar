use radar_toolbox::systems::physics::atmospherics::{clear_air_attenuation, rh_to_density};
pub fn solve(){
    println!("---- Solving problem 5 for chapter 4 ----");

    let vapor_density = rh_to_density(59.0, 0.0) * 273.15 / 216.7;

    let p_tot = 1013.25 - vapor_density;

    println!("Water vapor density {}", vapor_density);

    let f = clear_air_attenuation(10.0, p_tot, vapor_density, 273.15);


    println!("The propagation factor {:.6} dB/km", f);
}
