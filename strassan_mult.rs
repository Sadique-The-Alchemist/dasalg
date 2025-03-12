// helper function for standard 2 x2 multiplication
fn standard_multiply(a: &[[i32; 2]; 2], b: &[[i32; 2]; 2]) -> [[i32; 2]; 2] {
    return [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ];
}

// Strassan algiruthm for 2x2 matrixes
fn strassan_2x2(a: &[[i32; 2]; 2], b: &[[i32; 2]; 2]) -> [[i32; 2]; 2] {
    // compute the 7 products m1 to m7

    let m1 = (a[0][0] + a[1][1]) * (b[0][0] + b[1][1]);
    let m2 = (a[1][0] + a[1][1]) * b[0][0];
    let m3 = a[0][0] * (b[0][1] - b[1][1]);
    let m4 = a[1][1] * (b[1][0] - b[0][0]);
    let m5 = (a[0][0] + a[0][1]) * b[1][1];
    let m6 = (a[1][0] - a[0][0]) * (b[0][0] + b[0][1]);
    let m7 = (a[0][1] - a[1][1]) * (b[1][0] + b[1][1]);
    let c11 = m1 + m4 - m5 + m7;
    let c12 = m3 + m5;
    let c21 = m2 + m4;
    let c22 = m1 - m2 + m3 + m6;
    return [[c11, c12], [c21, c22]];
}

fn main() {
    let a = [[4, 5], [6, 8]];
    let b = [[10, 11], [12, 13]];
    let standard_result = standard_multiply(&a, &b);
    println!("Standard resulting matrix");
    for row in &standard_result {
        println!("{:?}", row);
    }

    let strassan_result = strassan_2x2(&a, &b);
    println!("Strassan Resulting matrix");
    for row in &strassan_result {
        println!("{:?}", row);
    }
}
