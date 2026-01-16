use radar_toolbox::signals::pulses::CW;
use radar_toolbox::signals::traits::RadarWaveform;
pub fn solve(){
    println!("---- Solving problem 5 for chapter 8 ----");

    let wave = CW::new(35E9, 1E-6);

    let ray_b = wave.rayleigh_bandwidth();

    println!("Rayleigh Resolution is {:.2e} Hz", ray_b);

}
