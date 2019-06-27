fn main() {
    let r1 = vec![10, 20, 50, 30];
    let r2 = vec![56, 83, 90, 60];
    let r3 = vec![70, 15, 52, 38];
    let r4 = vec![75, 11, 25, 99];
    let array = vec![r1, r2, r3, r4];

    let n = peak2d(array);
    println!("{:?}", n);
}

fn peak2d(array: Vec<Vec<u32>>) -> u32 {
    let m = array[0].len();
    let middle_column = m / 2;
    let mut middle_column_array: Vec<u32> = Vec::new();
    for item in array.iter() {
        if item.len() > 1{
            middle_column_array.push(item[middle_column - 1]);
        } else {
            middle_column_array.push(item[0]);
        }
    }
    let mut sorted = middle_column_array.clone();
    sorted.sort();
    let max_value = sorted.last().unwrap();
    println!("max element: {:?}", max_value);
    let index_max = middle_column_array
        .iter()
        .position(|s| s == max_value)
        .unwrap();
    println!("index of max element: {}", index_max);
    if middle_column > 0 && array[index_max][middle_column] < array[index_max][middle_column - 1] {
        let mut sliced_array = vec![];
        for i in &array {
            &sliced_array.push(i[..middle_column].to_vec());
        }
        println!("sliced array: {:?}", sliced_array);
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
