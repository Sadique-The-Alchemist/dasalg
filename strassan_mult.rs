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

// Strassan algiruthm for 2x2 matrixes
fn strassan_2x2(a: &[[i32; 2]; 2],  b: &[[i32; 2]; 2]) -> [[i32; 2]; 2]{
    // compute the 7 products m1 to m7 

    let s1 = b[0][1] - b[1][1];
    let s2 = a[0][0] + a[0][1];
    let s3 = a[1][0] + a[1][1];
    let s4 = b[1][0] - b[0][0];
    let s5 = a[0][0] + a[1][1];
    let s6 = b[0][0] + b[1][1];
    let s7 = b[0][1] - b[1][1];
    let s8 = b[1][0] + b[1][1];
    let s9 = a[0][0] - a[1][0];
    let s10 = b[0][0] + b[0][1];

    let p1 = a[0][0] * s1 
    let p2 = 
}