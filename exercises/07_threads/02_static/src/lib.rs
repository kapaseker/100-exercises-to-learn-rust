// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;
use std::thread::JoinHandle;

pub fn sum(v: &'static [i32]) -> i32 {
    if v.len() > 2 {
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
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
