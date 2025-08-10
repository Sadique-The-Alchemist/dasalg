fn find_max(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        panic!("Array cannot be empty");
    }
    let mut max_value = arr[0];
    for &value in arr.iter() {
        if value > max_value {
            max_value = value;
        }
    }
    return max_value;
}
fn main() {
    let numbers = [3, 5, 7, 2, 8];
    let max_number = find_max(&numbers);
    println!("The maximum number is: {}", max_number);
}