use radar_toolbox::systems::radar_simple;
       
pub fn solve(){
    println!("---- Solving problem 7 for chapter 2 ----");

    let radar = radar_simple::SimpleRadarBuilder::new("Radar 1")
        .transmit_power(25E3)
        .center_frequency(9.4E9)
        .transmit_gain(10.0_f64.powf(36.0/10.0))
        .receiver_gain(10.0_f64.powf(36.0/10.0))
        .receiver_bandwidth(10E6)
        .noise_figure(10.0_f64.powf(3.2/10.0))
        .transmit_loss(10.0_f64.powf(2.1/10.0))
        .receiver_loss(10.0_f64.powf(4.3/10.0))
        .atmospheric_loss((0.12 * 0.0002303)/2.0)
        .build().unwrap();
    

    let snr = radar.snr(1.0, 36E3);
    let pr = radar.radar_range_equation(1.0, 36E3);


    println!("The received power for {} is {:.2e} Watts", radar.name, pr);
    println!("The snr for {} is {:.2} dB", radar.name, 10.0*snr.log10());

    let radar = radar_simple::SimpleRadarBuilder::new("Radar 2")
        .transmit_power(25E4)
        .center_frequency(9.4E9)
        .transmit_gain(10.0_f64.powf(31.0/10.0))
        .receiver_gain(10.0_f64.powf(31.0/10.0))
        .receiver_bandwidth(10E6)
        .noise_figure(10.0_f64.powf(3.2/10.0))
        .transmit_loss(10.0_f64.powf(2.1/10.0))
        .receiver_loss(10.0_f64.powf(4.3/10.0))
        .atmospheric_loss((0.12 * 0.0002303)/2.0)
        .build().unwrap();
    

    let snr = radar.snr(1.0, 36E3);
    let pr = radar.radar_range_equation(1.0, 36E3);


    println!("The received power for {} is {:.2e} Watts", radar.name, pr);
    println!("The snr for {} is {:.2} dB", radar.name, 10.0*snr.log10());
    
    let radar = radar_simple::SimpleRadarBuilder::new("Radar 3")
        .transmit_power(25E4)
        .center_frequency(2.8E9)
        .transmit_gain(10.0_f64.powf(31.0/10.0))
        .receiver_gain(10.0_f64.powf(31.0/10.0))
        .receiver_bandwidth(10E6)
        .transmit_loss(10.0_f64.powf(2.1/10.0))
        .receiver_loss(10.0_f64.powf(4.3/10.0))
        .noise_figure(10.0_f64.powf(2.7/10.0))
        .atmospheric_loss((0.12 * 0.0002303)/2.0)
        .build().unwrap();
    

    let snr = radar.snr(1.0, 36E3);
    let pr = radar.radar_range_equation(1.0, 36E3);

    println!("The received power for {} is {:.2e} Watts", radar.name, pr);
    println!("The snr for {} is {:.2} dB", radar.name, 10.0*snr.log10());
    
    let radar = radar_simple::SimpleRadarBuilder::new("Radar 4")
        .transmit_power(25E4)
        .center_frequency(9.4E9)
        .transmit_gain(10.0_f64.powf(36.0/10.0))
        .receiver_gain(10.0_f64.powf(36.0/10.0))
        .receiver_bandwidth(10E6)
        .noise_figure(10.0_f64.powf(3.2/10.0))
        .transmit_loss(10.0_f64.powf(2.1/10.0))
        .receiver_loss(10.0_f64.powf(4.3/10.0))
        .atmospheric_loss((0.12 * 0.0002303)/2.0)
        .build().unwrap();
    

    let snr = radar.snr(1.0, 36E3);
    let pr = radar.radar_range_equation(1.0, 36E3);


    println!("The received power for {} is {:.2e} Watts", radar.name, pr);
    println!("The snr for {} is {:.2} dB", radar.name, 10.0*snr.log10());


}
