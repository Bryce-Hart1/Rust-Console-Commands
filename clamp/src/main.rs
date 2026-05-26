use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: clamp <number> <min> <max>");
        std::process::exit(1);
    }

    let number: f64 = args[1].parse().expect("Please provide a valid number");
    let min: f64 = args[2].parse().expect("Please provide a valid min");
    let max: f64 = args[3].parse().expect("Please provide a valid max");

    if min > max {
        eprintln!("Error: min ({}) cannot be greater than max ({})", min, max);
        std::process::exit(1);
    }

    if number < min {
        println!("{} is below range, clamped to min: {}", number, min);
    } else if number > max {
        println!("{} is above range, clamped to max: {}", number, max);
    } else {
        println!("{} is already within range [{}, {}]", number, min, max);
    }
}