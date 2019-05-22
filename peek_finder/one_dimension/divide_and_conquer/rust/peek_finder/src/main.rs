use rand::Rng;
use std::iter::FromIterator;

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers_array = Vec::new();
    for _i in 0..10 {
        numbers_array.push(rng.gen_range(0, 100));
    }
    let mid_position = numbers_array.len() / 2;
    println!("Generated Array: {:?}", numbers_array);
    println!("{}", numbers_array[mid_position]);
    if numbers_array[mid_position] < numbers_array[mid_position - 1] {
        let search_array = Vec::from_iter(numbers_array[..mid_position + 1].iter().cloned());
        let last_item = search_array.len() - 1;
        for i in 0..last_item {
            if i == 0 {
                if search_array[i] > search_array[i + 1] {
                    println!("Got the peek {}", search_array[i]);
                    break;
                }
            } else if i == last_item {
                if search_array[i] > search_array[i - 1] {
                    println!("Got the peek {}", search_array[i]);
                    break;
                }
            } else {
                if search_array[i] > search_array[i - 1] && search_array[i] > search_array[i + 1] {
                    println!("Got the peek {}", search_array[i]);
                    break;
                }
            }
        }
    } else if numbers_array[mid_position] < numbers_array[mid_position + 1] {
        let search_array = Vec::from_iter(numbers_array[mid_position + 1..].iter().cloned());
        let last_element = search_array.len() - 1;
        for i in 0..last_element {
            if i == 0 {
                if search_array[i] > search_array[i + 1] {
                    println!("Got the peek {}", search_array[i]);
                    break;
                }
            } else if i == last_element {
                if search_array[i] > search_array[i - 1] {
                    println!("Got the peek {}", search_array[i]);
                    break;
                }
            } else {
                if search_array[i] > search_array[i - 1] && search_array[i] > search_array[i + 1] {
                    println!("Got the peek {}", search_array[i]);
                    break;
                }
            }
        }
    } else {
        println!("got the peak in the middle");
    }
}
