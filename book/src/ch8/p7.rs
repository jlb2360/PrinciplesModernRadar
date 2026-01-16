use radar_toolbox::signals::pulses::CW;
use radar_toolbox::systems::pulse_radar::PulsedRadar;
pub fn solve(){
    println!("---- Solving problem 7 for chapter 8 ----");

    let wave = CW::new(35E9, 10E-6);

    let radar = PulsedRadar::new(wave, 20, 1E-3);

    let cpi = radar.cpi();

    let delta_dop = radar.doppler_resolution();

    println!("The coherent processing interval is {:.2} s", cpi);
    println!("The doppler resolution is {:.2} Hz", delta_dop);

}
