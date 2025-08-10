fn rotate(arr: &mut [i32], start: usize, end: usize)  {
    let mut start = start;
    let mut end = end;
    while start < end {
        arr.swap(start, end);
        start +=1;
        end -= 1;
    }
}

fn rotate_array(arr: &mut [i32], k: usize) {
    let n = arr.len();
    if n == 0 || k % n == 0 {
        return; 
    }
    let k = k % n; 
    rotate(arr, 0, n-1);
    rotate(arr, 0, k-1);
    rotate(arr, k, n-1);

}


fn main() {
    let mut arr = [1,2,3,4,5,6,7];
    let k = 2;
    println!("Original array: {:?}", arr);
    rotate_array(&mut arr, k);
    println!("Rotated array: {:?}", arr);
}