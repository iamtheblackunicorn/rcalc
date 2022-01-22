// RCALC by Alexander Abraham,
// a.k.a. "The Black Unicorn"
// a.k.a. "angeldustduke.eth".
// Licensed under the MIT license.

// Declaring a module for
// useful helper methods.
pub mod tools {

    // A standard error message for any erroneous behavior.
    pub fn error(){
        let error_msg = String::from("Wrong usage!");
        println!("{}", error_msg);
    }

    // Checking if the user-supplied input can be
    // converted to a float.
    pub fn is_num(num: String) -> bool {
        let mut result = false;
        if Some(num.parse::<f64>().unwrap()) != None {
            result = true;
        }
        else{
            // Do nothing.
        }
        return result;
    }

    // Checking if the right operators were supplied.
    pub fn is_operator(operator: String) -> bool{
        let mut result = false;
        if operator == "+" {
            result = true;
        }
        else if operator == "-" {
            result = true;
        }
        else if operator == "/" {
            result = true;
        }
        else if operator == "x" {
            result = true;
        }
        else {
            // Do nothing.
        }
        return result;
    }

    // Method to convert a string to a float.
    pub fn convert_to_float(num: String) -> f64 {
        let result = num.parse::<f64>().unwrap();
        return result;
    }
}
