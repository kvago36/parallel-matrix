use parallel::{multiply_matrices, parallel_multiply, rayon_multiply};
use rand::Rng;
use std::time::Instant;

fn generate_large_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::rng();
    let mut matrix = Vec::with_capacity(rows);

    for _ in 0..rows {
        let row: Vec<i32> = (0..cols).map(|_| rng.random_range(0..100)).collect();
        matrix.push(row);
    }

    matrix
}

fn main() {
    // for performance tests
    // let matrix_a = generate_large_matrix(100, 120);
    // let matrix_b = generate_large_matrix(120, 100);

    let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];

    let start = Instant::now();
    let result1 = multiply_matrices(&a, &b);
    let duration = start.elapsed();
    println!("Time elapsed with single thread is: {:?}", duration);

    // for performance tests
    // let a_clone = matrix_a.clone();
    // let b_clone = matrix_b.clone();

    let a_clone = a.clone();
    let b_clone = b.clone();

    let start = Instant::now();
    let result2 = parallel_multiply(a_clone, b_clone);
    let duration = start.elapsed();
    println!("Time elapsed with multiple threads is: {:?}", duration);

    let start = Instant::now();
    let result3 = rayon_multiply(a, b);
    let duration = start.elapsed();
    println!("Time elapsed with Ryaon is: {:?}", duration);

    assert_eq!(result1, result2);
    assert_eq!(result2, result3);

    println!("Result matrix: {:?}", result1);
}
