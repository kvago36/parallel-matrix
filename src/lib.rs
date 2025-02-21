use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn rayon_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    let mut result = vec![vec![0; cols_b]; rows_a];

    result.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..cols_b {
            row[j] = (0..cols_a).map(|k| a[i][k] * b[k][j]).sum();
        }
    });

    result
}

pub fn parallel_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    assert_eq!(
        cols_a,
        b.len(),
        "Число столбцов A должно совпадать с числом строк B"
    );

    let res = Arc::new(Mutex::new(vec![vec![0; cols_b]; rows_a]));

    let mut threads = vec![];

    for i in 0..rows_a {
        let res = Arc::clone(&res);
        let a_clone = a.clone();
        let b_clone = b.clone();

        threads.push(thread::spawn(move || {
            let mut vec1 = vec![0; cols_b];

            for j in 0..cols_b {
                for k in 0..cols_a {
                    vec1[j] += a_clone[i][k] * b_clone[k][j];
                }
            }

            res.lock().unwrap()[i] = vec1;
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    Arc::try_unwrap(res).unwrap().into_inner().unwrap()
}

pub fn multiply_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    assert_eq!(
        cols_a,
        b.len(),
        "Число столбцов A должно совпадать с числом строк B"
    );

    let mut result = vec![vec![0; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_matrices_done_right() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];

        let result = multiply_matrices(&a, &b);
        let calculated = vec![vec![58, 64], vec![139, 154]];

        assert_eq!(result, calculated);
    }

    #[test]
    fn rayon_multiply_matrices_done_right() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];

        let result = rayon_multiply(a, b);
        let calculated = vec![vec![58, 64], vec![139, 154]];

        assert_eq!(result, calculated);
    }

    #[test]
    fn parallel_multiply_matrices_done_right() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let b = vec![vec![4, 7, 3], vec![1, 5, 2], vec![8, 9, 6]];

        let result = parallel_multiply(a, b);
        let calculated = vec![vec![30, 44, 25], vec![69, 107, 58], vec![108, 170, 91]];

        assert_eq!(result, calculated);
    }
}
