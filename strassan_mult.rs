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

fn strassen_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
    // Base case: if 2x2, use Strassen's 2x2 method
    if n <= 2 {
        let a_2x2 = [[a[0][0], a[0][1]], [a[1][0], a[1][1]]];
        let b_2x2 = [[b[0][0], b[0][1]], [b[1][0], b[1][1]]];
        let result = if n == 1 {
            standard_multiply(&a_2x2, &b_2x2)
        } else {
            strassen_2x2(&a_2x2, &b_2x2)
        };
        return vec![
            vec![result[0][0], result[0][1]],
            vec![result[1][0], result[1][1]],
        ];
    }

    // Split matrices into quadrants
    let half = n / 2;
    let mut a11 = vec![vec![0; half]; half];
    let mut a12 = vec![vec![0; half]; half];
    let mut a21 = vec![vec![0; half]; half];
    let mut a22 = vec![vec![0; half]; half];
    let mut b11 = vec![vec![0; half]; half];
    let mut b12 = vec![vec![0; half]; half];
    let mut b21 = vec![vec![0; half]; half];
    let mut b22 = vec![vec![0; half]; half];
    for i in 0..half {
        for j in 0..half {
            a11[i][j] = a[i][j];
            a12[i][j] = a[i][j + half];
            a21[i][j] = a[i + half][j];
            a22[i][j] = a[i + half][j + half];
            b11[i][j] = b[i][j];
            b12[i][j] = b[i][j + half];
            b21[i][j] = b[i + half][j];
            b22[i][j] = b[i + half][j + half];
        }
    }

    // Compute the 7 products recursively
    let m1 = strassen_multiply(&add_matrix(&a11, &a22), &add_matrix(&b11, &b22), half);
    let m2 = strassen_multiply(&add_matrix(&a21, &a22), &b11, half);
    let m3 = strassen_multiply(&a11, &sub_matrix(&b12, &b22), half);
    let m4 = strassen_multiply(&a22, &sub_matrix(&b21, &b11), half);
    let m5 = strassen_multiply(&add_matrix(&a11, &a12), &b22, half);
    let m6 = strassen_multiply(&sub_matrix(&a21, &a11), &add_matrix(&b11, &b12), half);
    let m7 = strassen_multiply(&sub_matrix(&a12, &a22), &add_matrix(&b21, &b22), half);

    // Compute the result quadrants
    let c11 = add_matrix(&sub_matrix(&add_matrix(&m1, &m4), &m5), &m7);
    let c12 = add_matrix(&m3, &m5);
    let c21 = add_matrix(&m2, &m4);
    let c22 = add_matrix(&add_matrix(&sub_matrix(&m1, &m2), &m3), &m6);
    // Combine quadrants into result matrix
    let mut result = vec![vec![0; n]; n];
    for i in 0..half {
        for j in 0..half {
            result[i][j] = c11[i][j];
            result[i][j + half] = c12[i][j];
            result[i + half][j] = c21[i][j];
            result[i + half][j + half] = c22[i][j];
        }
    }

    result
}

// Helper function to add two matrices
fn add_matrix(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[i][j] + b[i][j];
        }
    }
    result
}
fn sub_matrix(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[i][j] - b[i][j];
        }
    }
    result
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

    let a = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    let b = vec![
        vec![17, 18, 19, 20],
        vec![21, 22, 23, 24],
        vec![25, 26, 27, 28],
        vec![29, 30, 31, 32],
    ];

    let result = strassen_multiply(&a, &b, 4);

    // Print the result
    println!("Resulting matrix:");
    for row in &result {
        println!("{:?}", row);
    }
}
