use std::env;

fn main() {
    //collect all 
    let args: Vec<String> = env::args().collect();

    //Check if the user provided at least one number, 
    //if they didn't then we cant calculate. Went off the same logic as the date example
    if args.len() < 2 {
        eprintln!("Must enter at least one number for the mean to be calculated");
        std::process::exit(1);
    }

    //Loop through each argument after the program name and convert to i32
    //if not then we print to the user to enter whole numbers
    let mut sum: i32 = 0;
    let count = (args.len() - 1) as i32;

    for i in 1..args.len() {
        let num: i32 = args[i].parse().expect("Please provide valid, whole numbers");
        sum += num;
    }

    let mean = sum / count;

    println!("Mean: {}", mean);
}