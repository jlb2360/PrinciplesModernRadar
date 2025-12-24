use std::env;

mod ch2;

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

        _ => eprintln!("Problem '{}' not found or not implemented yet.", target),
    }
}
