/// Doubles all values in the given vector that have odd indices.
/// 
/// # Panic
/// * Panics if a vector of length 1 or 0 is passed since that defeats
/// the purpose of this function.
pub fn double_odd(mut vector: Vec<i32>) -> Vec<i32>{
    // Panic if a vector of one or zero elements is passed, since the purpose
    // of the function is to double the *second* value.
    if vector.len() < 2 {
        panic!("ValueError: Vector should contain least two values!");
    }

    // Iterate through the given vector, doubling each second value
    // in-place.
    for i in 0..vector.len() {
        if i % 2 != 0 {
            vector[i] = vector[i] * 2
        }
    }
    vector
}

/// Test to confirm that the second value is indeed doubled.
#[test]
fn test_double_odd_doubles_second_value_only() {
    let test_vec = vec![1, 2, 3];
    let doubled_test_vec = vec![1, 4, 3];
    assert_eq!(double_odd(test_vec), doubled_test_vec);
}


/// Tests to confirm that the function panics for vectors of length 1.
#[test]
#[should_panic(expected = "ValueError: Vector should contain least two values!")]
fn test_double_odd_panics_if_empty() {
    let empty_vec: Vec<i32> = vec![];
    double_odd(empty_vec);
}

/// Tests to confirm that the function panics for vectors of length 0.
#[test]
#[should_panic(expected = "ValueError: Vector should contain least two values!")]
fn test_double_odd_panics_if_single_value() {
    let single_value_vec: Vec<i32> = vec![1];
    double_odd(single_value_vec);
}