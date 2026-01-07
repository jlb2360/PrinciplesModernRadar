use radar_toolbox::systems::physics::atmospherics::{clear_air_attenuation, rh_to_density, dust_attenuation};


pub fn solve(){
    println!("---- Solving problem 6 for chapter 4 ----");

    let vapor_density = rh_to_density(59.0, 0.0) * 273.15 / 216.7;

    let p_tot = 1013.25 - vapor_density;

    println!("Water vapor density {}", vapor_density);

    let f_air = clear_air_attenuation(10.0, p_tot, vapor_density, 273.15);

    let f_dust = dust_attenuation(1.0, 0.0001);

    let f = f_air * 8.0 + f_dust * 2.0;


    println!("The propagation loss {:.6} dB", f);
}
