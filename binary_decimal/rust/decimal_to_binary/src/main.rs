use std::env;

///  Basic function to get the binary from a decimal number

fn main() {
    let arg = env::args().nth(1).expect("no pattern given");
    let mut decimal_number: i32 = arg.parse().unwrap();
    // let mut binary_repr: Vec<i32> = vec![];
    let mut binary_repr = String::new();

    while decimal_number > 0 {
        //println!("argument {}", decimal_number);
        let remainder = decimal_number % 2;
        binary_repr.push_str(&remainder.to_string());
        decimal_number = decimal_number / 2;
    }

    binary_repr.chars().rev().collect::<String>();
    // match bits.chars().rev().collect::<String>().parse() {
    //         Ok(num) => num,
    //         Err(_e) => panic!("Something went wrong"),
    //     }

    println!("argument {}", binary_repr);
}
