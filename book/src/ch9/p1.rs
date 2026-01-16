use radar_toolbox::systems::physics::simple::propogation;
pub fn solve(){
    println!("---- Solving problem 1 for chapter 9 ----");

    let pt = 10.0;
    let r = 1E3;

    let pd_targ = propogation(pt, r);
    let pd_clutter = propogation(pt, r);

    println!("The power density at the target is {:.2e} W", pd_targ);
    println!("The power density at the clutter is {:.2e} W", pd_clutter);


}
