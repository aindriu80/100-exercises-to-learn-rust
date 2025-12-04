// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let len = v.len();
    if len == 0 {
        return 0;
    }
    let mid = len / 2;

    // Split the vector into two slices
    let (left, right) = v.split_at(mid);

    // Convert slices to owned Vecs for moving into threads
    let left = left.to_vec();
    let right = right.to_vec();

    let handle1 = thread::spawn(move || left.iter().sum::<i32>());

    let handle2 = thread::spawn(move || right.iter().sum::<i32>());

    // Wait for both thread and sum their results
    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();

    sum1 + sum2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
