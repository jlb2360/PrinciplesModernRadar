use crate::core::constants::{SpectroscopicLine, OXYGEN_LINES, WATER_VAPOUR_LINES};

// calculates attenuation due to dust

pub fn dust_attenuation(eta: f64, m: f64)->f64{
    4343.0 * eta * m
}


/// Calculates specific attenuation in dB/km using ITU-R P.676-13 Annex 1.
/// 
/// # Arguments
/// * `f` - Frequency in GHz
/// * `p` - Dry air pressure in hPa
/// * `e` - Water vapour partial pressure in hPa
/// * `temp_k` - Temperature in Kelvin
pub fn clear_air_attenuation(f: f64, p: f64, e: f64, temp_k: f64) -> f64 {
    let theta = 300.0 / temp_k; // [cite: 150]

    // 1. Calculate imaginary parts of complex refractivities
    let n_double_prime_oxygen = calculate_n_oxygen(f, p, e, theta);
    let n_double_prime_water_vapour = calculate_n_water_vapour(f, p, e, theta);

    // 2. Final specific attenuation formula 
    0.1820 * f * (n_double_prime_oxygen + n_double_prime_water_vapour)
}

fn calculate_n_oxygen(f: f64, p: f64, e: f64, theta: f64) -> f64 {
    let mut sum_lines = 0.0;
    
    // Iterating over oxygen spectroscopic data (Table 1)
    for line in OXYGEN_LINES.iter() {
        // Line strength [cite: 146]
        let s_i = line.a1_b1 * 1e-7 * p * theta.powi(3) * (line.a2_b2 * (1.0 - theta)).exp();
        
        // Line width with Zeeman splitting [cite: 218, 224]
        let delta_f_pure = line.a3_b3 * 1e-4 * (p * theta.powf(0.8 - line.a4_b4) + 1.1 * e * theta);
        let delta_f = (delta_f_pure.powi(2) + 2.25e-6).sqrt();
        
        // Correction factor for interference [cite: 229]
        let delta = (line.a5_b5 + line.a6_b6 * theta) * 1e-4 * (p + e) * theta.powf(0.8);
        
        sum_lines += s_i * line_shape_factor(f, line.f0, delta_f, delta);
    }

    // Add dry continuum (Debye spectrum + nitrogen) [cite: 141, 251]
    let d = 5.6e-4 * (p + e) * theta.powf(0.8); // Width parameter [cite: 254]
    let n_d = f * p * theta.powi(2) * (
        (6.14e-5 / (d * (1.0 + (f / d).powi(2)))) + 
        (1.4e-12 * p * theta.powf(1.5) / (1.0 + 1.9e-5 * f.powf(1.5)))
    );

    sum_lines + n_d
}

fn calculate_n_water_vapour(f: f64, p: f64, e: f64, theta: f64) -> f64 {
    let mut sum_lines = 0.0;
    
    // Iterating over water vapour spectroscopic data (Table 2)
    for line in WATER_VAPOUR_LINES.iter() {
        // Line strength [cite: 147]
        let s_i = line.a1_b1 * 1e-1 * e * theta.powf(3.5) * (line.a2_b2 * (1.0 - theta)).exp();
        
        // Line width with Doppler broadening [cite: 218, 224]
        let delta_f_pure = line.a3_b3 * 1e-4 * (p * theta.powf(line.a4_b4) + line.a5_b5 * e * theta.powf(line.a6_b6));
        let delta_f = 0.535 * delta_f_pure + (0.217 * delta_f_pure.powi(2) + (2.1316e-12 * line.f0.powi(2) / theta)).sqrt();
        
        let delta = 0.0; // Factor delta is 0 for water vapour [cite: 232]
        
        sum_lines += s_i * line_shape_factor(f, line.f0, delta_f, delta);
    }
    
    sum_lines
}

fn line_shape_factor(f: f64, fi: f64, delta_f: f64, delta: f64) -> f64 {
    // Formula for Fi [cite: 216]
    (f / fi) * (
        (delta_f - delta * (fi - f)) / ((fi - f).powi(2) + delta_f.powi(2)) +
        (delta_f - delta * (fi + f)) / ((fi + f).powi(2) + delta_f.powi(2))
    )
}


/// Converts Relative Humidity to Water Vapor Density (g/m^3)
/// 
/// # Arguments
/// * `rh` - Relative Humidity as a percentage (0.0 to 100.0)
/// * `temp_c` - Temperature in Celsius
pub fn rh_to_density(rh: f64, temp_c: f64) -> f64 {
    // 1. Calculate Saturation Vapor Pressure (hPa) 
    // using Magnus-Tetens approximation
    let es = 6.112 * ((17.62 * temp_c) / (temp_c + 243.12)).exp();

    // 2. Calculate Actual Vapor Pressure (hPa)
    let e = es * (rh / 100.0);

    // 3. Convert to Density (g/m^3) using Ideal Gas Law
    // Constant 216.74 accounts for gas constant Rv and unit conversions
    let density = (e * 216.74) / (temp_c + 273.15);

    density
}
