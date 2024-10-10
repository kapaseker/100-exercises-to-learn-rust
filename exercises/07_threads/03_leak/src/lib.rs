// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;
use std::thread::JoinHandle;

pub fn sum(v: Vec<i32>) -> i32 {

    if v.len() > 2 {

        let v = v.leak();

        let mid = v.len() / 2;
        let left = &v[0..mid];
        let right = &v[mid..];

        let left_thread: JoinHandle<i32> = thread::spawn(|| {
            left.iter().sum()
        });

        let right_thread: JoinHandle<i32> = thread::spawn(|| {
            right.iter().sum()
        });

        left_thread.join().unwrap() + right_thread.join().unwrap()
    } else {
        v.iter().sum()
    }
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
