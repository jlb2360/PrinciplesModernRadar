use radar_toolbox::targets::clutter::critical_gr;
use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 5 for chapter 5 ----");

    let l_x = C / 10E9;
    let l_l = C / 1E9;
    let l_k = C / 35E9;
    let sig_h1 = 1E-2;
    let sig_h2 = 10E-2;
    
    let mut gr_crit = critical_gr(sig_h1, l_x);
    println!("critical grazing angle for X band and 1.0 cm roughness is {:.2} degree", gr_crit.to_degrees());
    
    gr_crit = critical_gr(sig_h2, l_x);
    println!("critical grazing angle for X band and 10.0 cm roughness is {:.2} degree", gr_crit.to_degrees());

    gr_crit = critical_gr(sig_h1, l_l);
    println!("critical grazing angle for L band and 1.0 cm roughness is {:.2} degree", gr_crit.to_degrees());

    gr_crit = critical_gr(sig_h2, l_l);
    println!("critical grazing angle for L band and 10.0 cm roughness is {:.2} degree", gr_crit.to_degrees());

    gr_crit = critical_gr(sig_h1, l_k);
    println!("critical grazing angle for K band and 1.0 cm roughness is {:.2} degree", gr_crit.to_degrees());

    gr_crit = critical_gr(sig_h2, l_k);
    println!("critical grazing angle for K band and 10.0 cm roughness is {:.2} degree", gr_crit.to_degrees());



}
