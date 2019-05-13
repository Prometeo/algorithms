use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers_array = Vec::new();
    for _i in 0..10 {
        numbers_array.push(rng.gen_range(0, 100));
    }
    let last_item = numbers_array.len();
    println!("Generated Array: {:?}", numbers_array);
    for i in 0..last_item {
        if i == 0 {
            if numbers_array[i] > numbers_array[i + 1] {
                println!("Got the peek {} in {} steps", numbers_array[i], i + 1);
                break;
            }
        } else if i == last_item {
            if numbers_array[i] > numbers_array[i - 1] {
                println!("Got the peek {} in {} steps", numbers_array[i], i + 1);
                break;
            }
        } else {
            if numbers_array[i] > numbers_array[i + 1] && numbers_array[i] > numbers_array[i - 1] {
                println!("Got the peek {} in {} steps", numbers_array[i], i + 1);
                break;
            }
        }
    }
}
