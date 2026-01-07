use radar_toolbox::targets::{target_traits::Pfa, thermal_noise::ThermalNoise};         
pub fn solve(){
    println!("---- Solving problem 6 for chapter 3 ----");

    let noise = ThermalNoise::new(10.0, 150E-3);

    let threshold = noise.threshold(1E-4) * 1E3;
    
    println!("Threshold requeired for 1E-4 pfa: {:.2} mV", threshold);
}
