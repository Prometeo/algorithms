use std::env;

/// Basic function to get the decimal repr from a binary number

fn main() {
    let arg = env::args().nth(1).expect("no pattern given");
    let binary_chars: Vec<_> = arg.split("").collect();
    let mut decimal_var = 0;

    for i in binary_chars {
        let int_char: u32 = match i.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        decimal_var = 2 * decimal_var + int_char;
    }

    println!("{}",decimal_var);
}
