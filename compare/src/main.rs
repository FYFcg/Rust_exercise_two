use std::io::{self};

fn compare_string(x: &str, y: &str) -> bool {
    let x_bytes = x.as_bytes();
    let y_bytes = y.as_bytes();

    let min_len = std::cmp::min(x_bytes.len(), y_bytes.len());

    for i in 0..min_len {
        if x_bytes[i] > y_bytes[i] {
            return true;
        } else if x_bytes[i] < y_bytes[i] {
            return false;
        }
    }

    x_bytes.len() > y_bytes.len()
}

fn main() {
    let mut input_x = String::new();
    let mut input_y = String::new();

    println!("Enter string x: ");
    io::stdin().read_line(&mut input_x).unwrap();
    let x = input_x.trim();

    println!("Enter string y: ");
    io::stdin().read_line(&mut input_y).unwrap();
    let y = input_y.trim();

    let result = compare_string(x, y);

    println!("Result: {}", result);
}
