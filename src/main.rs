use double_odd::double_odd;
use std::env;

mod double_odd;
/// The main implementation of this application.
/// Defines a new array and uses the [double_sum()]
/// to double each item in the defined array.
fn main() {
    // Get the numbers as arguments passed to the 
    let values: Vec<String> =  env::args().skip(1).map(|x| x.parse().unwrap()).collect();
    if values.len() < 2 {
        println!("Please enter 2 or more integers.")
    } else {
        let numbers: Vec<i32> = str_vec_to_int(values);
    
        println!("{:?}", double_odd(numbers));
    }
}

/// Converts the vector passed to a vector of integers.
/// 
/// # Panics
/// Panics if a value in `vector` is not a valid i32 integer.
fn str_vec_to_int(vector: Vec<String>) -> Vec<i32> {
    let values: Vec<i32> = vector.iter().map(|x| match x.parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            panic!("TypeError: {} is not an integer!", x);
        }
    }).collect();
    values
}