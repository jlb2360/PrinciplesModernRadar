use std::f64::consts::PI;

// calculations done in power
pub fn transmission_f(
    current_power: f64,
    gt: f64
) -> f64 {
    current_power * gt
}

pub fn propogation(
    current_power: f64,
    range: f64
) -> f64 {
    current_power/(4.0 * PI * range * range)
}

pub fn atm_attenuation(
    current_power: f64,
    range: f64,
    loss_one_way_db_km: f64
) -> f64{
    if loss_one_way_db_km==0.0{
        return current_power;
    }
    let loss = range*1E-3 * loss_one_way_db_km;
    current_power/(10.0_f64.powf(loss/10.0))
}

pub fn reflection(
    current_power: f64,
    rcs: f64
) -> f64 {
    current_power*rcs
}

pub fn reception(
    current_power: f64,
    lambda: f64,
    gr: f64
)->f64{
    current_power * gr * lambda * lambda / (4.0 * PI)
}

pub fn reception_ae(
    current_power: f64,
    ae: f64
)->f64{
    current_power*ae
}

pub fn chain_loss(
    current_power: f64,
    loss: f64
)->f64{
    current_power/loss
}
