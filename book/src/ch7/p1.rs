use radar_toolbox::targets::simple::*;
use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 1 for chapter 7 ----");

    let lam = C/10E9;
    let a_s = 1.0;

    let sp = Sphere::from_radius(a_s);
    let cr = CornerReflector::from_rcs(sp.rcs, lam);

    let frac_as_at = sp.a/cr.a;

    println!("The rcs from the sphere is {:.2} dBsm", 10.0*sp.rcs.log10());

    println!("The length for a corner reflector with the same rcs is {:.2} m", cr.a);

    println!("The ratio of radius to length is {:.2}", frac_as_at);


}
