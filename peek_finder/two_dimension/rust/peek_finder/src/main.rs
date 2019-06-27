use rand::Rng;

fn main() {
    let mut r1 = Vec::new();
    let mut r2 = Vec::new();
    let mut r3 = Vec::new();
    let mut r4 = Vec::new();
    rnd_array(&mut r1);
    rnd_array(&mut r2);
    rnd_array(&mut r3);
    rnd_array(&mut r4);
    let array = vec![r1, r2, r3, r4];

    let n = peak2d(array);
    println!("{:?}", n);
}

fn rnd_array(array: &mut Vec<u32>){
    let mut rng = rand::thread_rng();
    for _i in 0..4 {
        array.push(rng.gen_range(0, 99));
    }
}
fn peak2d(array: Vec<Vec<u32>>) -> u32 {
    let m = array[0].len();
    let middle_column = m / 2;
    let mut middle_column_array: Vec<u32> = Vec::new();
    for item in array.iter() {
        if item.len() > 1 {
            middle_column_array.push(item[middle_column - 1]);
        } else {
            middle_column_array.push(item[0]);
        }
    }
    let mut sorted = middle_column_array.clone();
    sorted.sort();
    let max_value = sorted.last().unwrap();
    let index_max = middle_column_array
        .iter()
        .position(|s| s == max_value)
        .unwrap();
    if middle_column > 0 && array[index_max][middle_column] < array[index_max][middle_column - 1] {
        let mut sliced_array = vec![];
        for i in &array {
            &sliced_array.push(i[..middle_column].to_vec());
        }
        peak2d(sliced_array)
    } else if middle_column < m - 1
        && array[index_max][middle_column] < array[index_max][middle_column + 1]
    {
        let mut sliced_array = vec![];
        for i in &array {
            sliced_array.push(i[middle_column..].to_vec());
        }
        println!("sliced array: {:?}", sliced_array);
        peak2d(sliced_array)
    } else {
        array[index_max][middle_column]
    }
}
