use std::env;

mod ch2;
mod ch3;
mod ch4;
mod ch5;
mod ch7;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <chapter>.<problem>");
        eprintln!("Example: cargo run -- 1.2");
        return;
    }

    let target = &args[1];

    match target.as_str() {
        // Chapter 2
        "2.1" => ch2::p1::solve(),
        "2.2" => ch2::p2::solve(),
        "2.3" => ch2::p3::solve(),
        "2.4" => ch2::p4::solve(),
        "2.5" => ch2::p5::solve(),
        "2.6" => ch2::p6::solve(),
        "2.7" => ch2::p7::solve(),
        "2.9" => ch2::p9::solve(),
        "2.10" => ch2::p10::solve(),
        "2.11" => ch2::p11::solve(),
        "2.12" => ch2::p12::solve(),
        "2.13" => ch2::p13::solve(),
        "2.14" => ch2::p14::solve(),
        "2.15" => ch2::p15::solve(),
        "2.16" => ch2::p16::solve(),
        "2.17" => ch2::p17::solve(),

        // Chapter 3
        "3.1" => ch3::p1::solve(),
        "3.2" => ch3::p2::solve(),
        "3.3" => ch3::p3::solve(),
        "3.5" => ch3::p5::solve(),
        "3.6" => ch3::p6::solve(),
        "3.7" => ch3::p7::solve(),
        "3.8" => ch3::p8::solve(),
        "3.9" => ch3::p9::solve(),
        "3.10" => ch3::p10::solve(),
        "3.11" => ch3::p11::solve(),
        "3.12" => ch3::p12::solve(),
        "3.13" => ch3::p13::solve(),
        "3.14" => ch3::p14::solve(),
        "3.15" => ch3::p15::solve(),
        "3.16" => ch3::p16::solve(),

        // chapter 4
        "4.2" => ch4::p2::solve(),
        "4.3" => ch4::p3::solve(),
        "4.4" => ch4::p4::solve(),
        "4.5" => ch4::p5::solve(),
        "4.6" => ch4::p6::solve(),
        "4.9" => ch4::p9::solve(),
        "4.13" => ch4::p13::solve(),
        "4.15" => ch4::p15::solve(),
        "4.16" => ch4::p16::solve(),

        // chapter 5
        "5.1" => ch5::p1::solve(),
        "5.2" => ch5::p2::solve(),
        "5.5" => ch5::p5::solve(),
        "5.8" => ch5::p8::solve(),
 
        // chapter 7
        "7.1" => ch7::p1::solve(),
        "7.8" => ch7::p8::solve(),
        "7.9" => ch7::p9::solve(),
        "7.10" => ch7::p10::solve(),
        "7.11" => ch7::p11::solve(),
    

        _ => eprintln!("Problem '{}' not found or not implemented yet.", target),
    }
}
