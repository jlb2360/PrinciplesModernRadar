use radar_toolbox::antennas::phased_array;
use radar_toolbox::core::constants::C;
pub fn solve(){
    println!("---- Solving problem 9 for chapter 9 ----");

    let dx = phased_array::LinearArray::no_grating(65.0_f64.to_radians(), C/10E9);
    let n = phased_array::LinearArray::elements_needed_for_th3(1.0_f64.to_radians(), dx, C/10E9, 0.0);

    println!("The necessary spacing is {:.2} cm", dx*1E2);
    println!("The necessary number of elements is {}", n);


}

