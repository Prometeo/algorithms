use std::env;
///  Basic function to get the binary from a decimal number

fn main() {
    let arg = env::args().nth(1).expect("no pattern given");
    let mut decimal_number: i32 = arg.parse().unwrap();
    let mut binary_str = String::new();
    while decimal_number > 0 {
        let remainder = decimal_number % 2;
        binary_str.push_str(&remainder.to_string());
        decimal_number = decimal_number / 2;
    }
    let binary_result = binary_str.chars().rev().collect::<String>();
    println!("argument {}", binary_result);
}
