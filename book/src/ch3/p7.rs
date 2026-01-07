use radar_toolbox::targets::{target_traits::Pfa, thermal_noise::ThermalNoise};         
pub fn solve(){
    println!("---- Solving problem 7 for chapter 3 ----");

    let noise = ThermalNoise::new(10.0, 150E-3);

    let n_dwells = noise.n_dwells(1E-9, 1E-4); 
    
    println!("Number of dwells to get 1E-9 Pfa from 1E-4 Pfa: {}", n_dwells);
}
