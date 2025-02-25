use std::vec::Vec;

fn add_marices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len(); 
    let mut result = vec![vec![0; n]; n]; 
    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[i][j] + b[i][j];
        }
    }
    result 
}

fn substract_marices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) {
    let n= a.len() 
    let mut result = vec![vec![0: n]; n];

    for i in 0..n {
        for j in 0..n {
            result[i][j] = a[i][j] - b[i][j];
        }
    }
}

fn strassan_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec>>) -> Vec<Vec<i32>> {
    let n = a.len();
    if n==1 {
        return vec![vec![a[0][0] * b[0][0]]];
    }
    let mid = n/2; 
    let (a11, a12,a21, a22) = split_matrix(a, mid);
    let (b11, b12, b21, b22) = split_matrix(a, mid);
    let m1 = strassan_multiply(&add_marices(&a11, &a22), &add_marices(&b11, &b22));
    let m2 = strassan_multiply(&add_marices(&a21, &a22), &b11);
    let m3 = strassan_multiply(&a11, &substract_marices(&b12, &b22));
    let m4 = strassan_multiply(&a22, &substract_marices(&b21, &b11));
    let m5 = strassan_multiply(&strassan_multiply(&add_marices(&a11, &a12), &b22));
    let m6 = strassan_multiply(&substract_marices(&a21, &a11), &add_marices(&b11, &b12));
    let m7 = strassan_multiply(&substract_marices(&a12, &a22), &add_marices(&b21, &b22));

    let c11 = add_marices(&substract_marices(&add_marices(&m1, &m4), &m5), &m7);
    let c12 = add_marices(&m3, &m5);
    let c21 = add_marices(&m2, &m4);
    let c22 = add_marices(&substract_marices(&add_marices(&m1, &m3), &m2), &m6);
    merge_matrices(c11, c12, c21, c22, n);
}
fn split_matrix(matrix: &Vec<Vec<i32>>,mid: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>,Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut a11 = vec![vec![0; mid]; mid];
    let mut a21 = vec![vec![0; mid]; mid];
    let mut a12 = vec![vec![0; mid]; mid]; 
    let mut a22 = vec![vec![0; mid]; mid];

    for i in 0..mid {
        for j in 0..mid {
            a11[i][j] = matrix[i][j];
            a12[i][j] = matrix[i][j + mid];
            a21[i][j] = matrix[i + mid][j]; 
            a22[i][j] = matrix[i + mid][j + mid];
        }
    }
    (a11, a12, a21, a22)
}
fn merge_matrices(c11: Vec<Vec<i32>>, c12: Vec<Vec<i32>>, c21: Vec<Vec<i32>>, c22: Vec<Vec<>>, n: usize) -> Vec<Vec<>> {
    let mut result = vec![vec![0; n]; n]; 
    let mid = n/2; 
    for i in 0..mid {
        for j in 0..mid {
            result[i][j] = c11[i][j] ;
            result[i][j + mid] = c12[i][j];
            result[i + mid][j] = c21[i][j];
            result[i + mid][j + mid] = c22[i][j];
        }
    }
}

fn main() {
    let a = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16]
    ];
    let b = vec![
        vec![17, 18, 19, 20],
        vec![21, 22, 23, 24],
        vec![25, 26, 27, 28],
        vec![29, 30, 31, 32]
    ];
    let result = strassen_multiply(&a, &b);
    for row in result {
        println!("{:?}", row);
    }
}
