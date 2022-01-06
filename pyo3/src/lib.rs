extern crate serde;
extern crate serde_json;
use log::{debug, error, info, warn};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fmt;

/// Multiply two numbers:
#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

/// Return the sum of a list/vector of numbers
#[pyfunction]
fn list_sum(a: Vec<isize>) -> PyResult<isize> {
    let mut sum: isize = 0;
    for i in a {
        sum += i;
    }
    Ok(sum)
}

// /// Word printer:
// /// Prints a word to the console n number of times.
// /// Optionally, the word is printed in reverse and or in uppercase.
// #[pyfunction]
// fn word_printer(mut word: String, n: isize, reverse: bool, uppercase: bool) {
//     if reverse {
//         let mut reversed_word = String::new();
//         for c in word.chars().rev() {
//             reversed_word.push(c);
//         }
//         word = reversed_word;
//     }
//     if uppercase {
//         word = word.to_uppercase();
//     }
//     for _ in 0..n {
//         println!("{}", word);
//     }
// }

// /// Print every item of a list to console:
// #[pyfunction]
// fn vector_printer(a: Vec<String>) {
//     for string in a {
//         println!("{}", string)
//     }
// }

// Print all the key values in a dict to console:
// #[pyfunction]
// fn dict_printer(hm: HashMap<String, String>) {
//     for (key, value) in hm {
//         println!("{} {}", key, value)
//     }
// }

// /// Print every item in an array to console:
// #[pyfunction]
// fn array_printer(a: [String; 8]) {
//     for string in a {
//         println!("{}", string)
//     }
// }

#[pyfunction]
fn count_occurences(contents: &str, needle: &str) -> usize {
    let mut count = 0;
    for line in contents.lines() {
        for word in line.split(" ") {
            if word == needle || word == format!("{}.", needle) {
                count += 1;
            }
        }
    }
    count
}
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction] 
fn human_says_hi(human_data: String) -> PyResult<String> {
    // println!("{}", human_data);
    let human: Human = serde_json::from_str(&human_data).map_err(
        |e| {
            let error_message = format!("Failed to parse human_data: {}", e.to_string());
            PyValueError::new_err(error_message)
        }
    )?;
    Ok(human.name)
    // println!(
    //     "Now we can work with the struct:\n {:#?}.\n {} is {} years old.",
    //     human, human.name, human.age,
    // )
}

#[derive(Serialize, Deserialize)]
struct Human {
    name: String,
    age: u8,
}

// #[pyfunction]
// fn log_different_levels() {
//     error!("logging an error");
//     warn!("logging a warning");
//     info!("logging an info message");
//     debug!("logging a debug message");
// }

// #[pyfunction]
// fn log_example() {
//     info!("A log message from {}!", "Rust");
// }

#[pyfunction]
fn get_fibonacci(number: isize) -> PyResult<u128> {
    if number == 1 {
        return Ok(1);
    } else if number == 2 {
        return Ok(2);
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..number {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    Ok(sum)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pyo3_samples(_py: Python, m: &PyModule) -> PyResult<()> {
    // pyo3_log::init();
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    // m.add_function(wrap_pyfunction!(word_printer, m)?)?;
    // m.add_function(wrap_pyfunction!(vector_printer, m)?)?;
    // m.add_function(wrap_pyfunction!(dict_printer, m)?)?;
    // m.add_function(wrap_pyfunction!(array_printer, m)?)?;
    m.add_function(wrap_pyfunction!(count_occurences, m)?)?;
    m.add_function(wrap_pyfunction!(human_says_hi, m)?)?;
    // m.add_wrapped(wrap_pyfunction!(log_example))?;
    // m.add_wrapped(wrap_pyfunction!(log_different_levels))?;
    m.add_function(wrap_pyfunction!(get_fibonacci, m)?)?;
    // m.add_function(wrap_pyfunction!(greater_than_2, m)?)?;
    // m.add_class::<RustStruct>()?;

    Ok(())
}
