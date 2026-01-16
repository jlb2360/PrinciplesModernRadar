use radar_toolbox::signals::pulses::CW;
use radar_toolbox::signals::traits::RadarWaveform;
use radar_toolbox::systems::pulse_radar::PulsedRadar;
pub fn solve(){
    println!("---- Solving problem 9 for chapter 8 ----");

    let wave = CW::new(35E9, 10E-6);

    let radar = PulsedRadar::new(wave, 30, 100E-6);

    let range_res = radar.waveform.range_resolution();

    let delta_dop = radar.doppler_resolution();

    let r_amb = radar.range_ambiguity();
    let dop_amb = radar.doppler_ambiguity();

    println!("The range resolution is {:.2} m", range_res);
    println!("The doppler resolution is {:.2} Hz", delta_dop);
    println!("The Range ambiguity is {:.2} m", r_amb);
    println!("The doppler ambiguity is {:.2} Hz", dop_amb);

}
