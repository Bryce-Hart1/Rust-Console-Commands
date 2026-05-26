use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly 3 arguments, if not we cannot work with it
    if args.len() < 4 {
        eprintln!("Usage: clamp <number> <min clamp> <max clamp>");
        std::process::exit(1);
    }

    //check if they are valid or not
    let number: f64 = args[1].parse().expect("Please provide a valid number");
    let min: f64 = args[2].parse().expect("Please provide a valid min");
    let max: f64 = args[3].parse().expect("Please provide a valid max");

    // Make sure min is not greater than max, and if it is, tell user
    if min > max {
        eprintln!("Error: min ({}) cannot be greater than max ({})", min, max);
        std::process::exit(1);
    }

    let result;
    if number < min{
        result = min;
    } else if number > max{
        result = max;
    } else{
        result = number;
    }

    println!("{} clamped to [{}, {}] = {}", number, min, max, result);
}