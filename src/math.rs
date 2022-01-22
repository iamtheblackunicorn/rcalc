// RCALC by Alexander Abraham,
// a.k.a. "The Black Unicorn"
// a.k.a. "angeldustduke.eth".
// Licensed under the MIT license.

// Declaring a module for
// mathematical functions.
pub mod functions{

    // Adds [num_one] to [num_two].
    pub fn add(num_one: f64, num_two: f64) -> f64 {
        return num_one + num_two;
    }

    // Subtracts [num_one] and [num_two].
    pub fn sub(num_one: f64, num_two: f64) -> f64 {
        return num_one - num_two;
    }

    // Multiplies [num_one] with [num_two].
    pub fn mul(num_one: f64, num_two: f64) -> f64 {
        return num_one * num_two;
    }

    // Divides [num_one] by [num_two].
    pub fn div(num_one: f64, num_two: f64) -> f64 {
        return num_one / num_two;
    }
}
