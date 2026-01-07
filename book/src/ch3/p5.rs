use radar_toolbox::targets::{target_traits::Pfa, thermal_noise::ThermalNoise};         
pub fn solve(){
    println!("---- Solving problem 5 for chapter 3 ----");

    let noise = ThermalNoise::new(10.0, 1.0);

    let pfa = noise.pfa(3.0);
    
    println!("Probability of False Aquisition {:.2e}", pfa);
}
