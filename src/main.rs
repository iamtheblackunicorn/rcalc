// RCALC by Alexander Abraham,
// a.k.a. "The Black Unicorn"
// a.k.a. "angeldustduke.eth".
// Licensed under the MIT license.

// Importing everything.
mod math;
mod utils;
use std::env;
use utils::tools::error;
use utils::tools::is_num;
use math::functions::add;
use math::functions::sub;
use math::functions::mul;
use math::functions::div;
use utils::tools::is_operator;
use utils::tools::convert_to_float;

// The app's main command-line interface.
fn cli(){

    // List or "vector" of arguments.
    let args: Vec<String> = env::args().collect();

    // Checking if the number
    // of arguments supplied is correct.
    if args.len() == 4{

        let arg_one = &args[1];
        let arg_two = &args[2];
        let arg_three = &args[3];

        // Checking if all input is valid.
        if is_num(arg_one.to_string()) == true && is_num(arg_two.to_string()) == true && is_operator(arg_three.to_string()) == true {

            // Addition.
            if args[3] == "+"{
                let num_one: f64 = convert_to_float(args[1].to_string());
                let num_two: f64 = convert_to_float(args[2].to_string());
                println!("{}", add(num_one, num_two));
            }

            // Subtraction.
            else if args[3] == "-"{
                let num_one: f64 = convert_to_float(args[1].to_string());
                let num_two: f64 = convert_to_float(args[2].to_string());
                println!("{}", sub(num_one, num_two));
            }

            // Multiplication.
            else if args[3] == "x"{
                let num_one: f64 = convert_to_float(args[1].to_string());
                let num_two: f64 = convert_to_float(args[2].to_string());
                println!("{}", mul(num_one, num_two));
            }

            // Division.
            else if args[3] == "/"{
                let num_one: f64 = convert_to_float(args[1].to_string());
                let num_two: f64 = convert_to_float(args[2].to_string());
                println!("{}", div(num_one, num_two));
            }
            else {
                error();
            }
        }
        else {
            error();
        }
    }
    else {
        error();
    }
}

// Main entry-point for the
// Rust compiler.
fn main(){
    cli();
}
