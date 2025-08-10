fn isort(arr: &mut [i32]) { 
    for i in 1..arr.len(){
        let key = arr[i];
        let mut j = i as isize - 1; 
        while j >= 0 && arr[j as usize] > key {
            arr[j as usize + 1] = arr[j as usize];
            arr[j as usize] = key;
            j -= 1;
        } 
    }

}

fn main() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    isort(&mut arr);
    println!("Sorted array: {:?}", arr);
}