use radar_toolbox::signals::pulses::CW;
use radar_toolbox::signals::traits::RadarWaveform;
pub fn solve(){
    println!("---- Solving problem 4 for chapter 8 ----");

    let wave = CW::new(10E9, 100E-9);

    let ray_b = wave.rayleigh_bandwidth();
    let b_3db = wave.bandwidth_3db();

    println!("Rayleigh Bandwidth {:.2e} Hz", ray_b);
    println!("3 dB Bandwidth {:.2e} Hz", b_3db);



}
