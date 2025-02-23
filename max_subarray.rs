// Define a struct to hold the result of the maximum sub array
struct MaxSubarray {
    sum: i32,     // The maximum sum
    left: usize,  // Start index of the subarray
    right: usize, // End index of the subarray
}
fn max_subarray(arr: &[i32]) -> MaxSubarray {
    // Base case if the array has 1 element, return it
    if arr.len() == 1 {
        return MaxSubarray {
            sum: arr[0],
            left: 0,
            right: 0,
        };
    }
    // Split the array into two halves
    let mid = arr.len() / 2;
    let left_half = &arr[..mid];
    let right_half = &arr[mid..];

    // Recursively find max subarray in lef and right half

    let left_result = max_subarray(left_half);
    let right_result = max_subarray(right_half);

    // find the max sub array that crosses the middle
    let cross_result = max_crossing_subarray(arr, mid);

    // Compare the three possibilities and return the response

    if left_result.sum >= right_result.sum && left_result.sum >= cross_result.sum {
        MaxSubarray {
            sum: left_result.sum,
            left: left_result.left,
            right: left_result.right,
        }
    } else if right_result.sum >= left_result.sum && right_result.sum >= cross_result.sum {
        MaxSubarray {
            sum: right_result.sum,
            left: mid + right_result.left,
            right: mid + right_result.right,
        }
    } else {
        cross_result
    }
}

// Helper function to find the max subarray crossing the middle

fn max_crossing_subarray(arr: &[i32], mid: usize) -> MaxSubarray {
    // find max sum on a left side of mid

    let mut left_sum = i32::MIN;
    let mut curr_sum = 0;
    let mut left_idx = mid;
    for i in (0..mid).rev() {
        curr_sum += arr[i];
        if curr_sum > left_sum {
            left_sum = curr_sum;
            left_idx = i;
        }
    }
    // Find max sum on right side of the mid
    let mut right_sum = i32::MIN;
    curr_sum = 0;
    let mut right_idx = mid;
    for i in mid..arr.len() {
        curr_sum += arr[i];
        if curr_sum > right_sum {
            right_sum = curr_sum;
            right_idx = i;
        }
    }

    MaxSubarray {
        sum: left_sum + right_sum,
        left: left_idx,
        right: right_idx,
    }
}
fn main() {
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = max_subarray(&arr);

    println!(
        "Maximum subarray sum: {} (from index {} to {})",
        result.sum, result.left, result.right
    );
    println!("Subarray: {:?}", &arr[result.left..=result.right]);
}
