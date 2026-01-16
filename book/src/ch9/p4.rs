pub fn solve(){
    println!("---- Solving problem 4 for chapter 9 ----");
    
    let d_max = 35.0; // dBi
    let a_e = -0.46 - 1.18; // dB
    let d_taylor = d_max + a_e; // dB
    let s_taylor = d_taylor - 25.0; // dBi
    
    println!("a) The peak sidelobe is 22.0 dB");
    println!("b) The peak sidelobe is {:.2} dB", s_taylor);
    println!("b) The new peax directivity is {:.2} dBi", d_taylor);

}
