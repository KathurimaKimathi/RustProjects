use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // The first argument is the location of the compiled binary, so skip it
    let first: String = args.nth(1).unwrap();
    // After accessing the second argument, the iterator's next element becomes the first
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

   // Parse strings to numbers
   let first_number = first.parse::<f32>().unwrap();
   let second_number = second.parse::<f32>().unwrap();

   // call operate function
   let result = operate(operator, first_number, second_number);

   println!("{}", output(first_number, operator, second_number, result));  
}

// Perform basic arithmetic operations
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => {
            first_number + second_number
        },
        '-' => {
            first_number - second_number
        },
        '/' => {
            first_number / second_number
        },
        '*' | 'X' | 'x' => {
            first_number * second_number
        },
        _ => panic!("Invalid operator used"),
    }
}

// function to output the results
fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}", first_number, operator, second_number, result
    )
}
