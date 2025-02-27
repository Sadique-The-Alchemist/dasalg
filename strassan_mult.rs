use std::opts::{Add, Sub, Mul} 
// helper function for standard 2 x2 multiplication
fn standard_multiply(a: &[[i32: 2]; 2], b: &[[i32: 2]; 2]) -> [[i32; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1]
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1]
        ]
    ]
}