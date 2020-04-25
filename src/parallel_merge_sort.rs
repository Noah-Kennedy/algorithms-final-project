use crate::OutOfPlaceSorterSorter;
#[cfg(test)]
use crate::test_utils::{check_out_of_place_sorter, TestNum};
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::thread;

pub struct ParallelMergeSorter {
    available_threads: AtomicUsize,
}

impl Default for ParallelMergeSorter {
    fn default() -> Self {
        ParallelMergeSorter {
            available_threads: AtomicUsize::new(num_cpus::get())
        }
    }
}

impl<T> OutOfPlaceSorterSorter<T> for ParallelMergeSorter where T: PartialOrd + Clone + Send + Sync {
    fn sort(&self, array: &[T]) -> Vec<T> {
        let len = array.len();
        if len <= 1 {
            array.to_vec()
        } else {
            let middle = array.len() / 2;
            let (left, right) = array.split_at(middle);

            let should_spawn = len >= 10_000 && self.available_threads.load(Ordering::SeqCst) > 0;

            // if we have available threads
            let (left, right) = if should_spawn {
                self.available_threads.fetch_sub(1, Ordering::SeqCst);
                // then we will spawn a new thread for the left side and sort both in parallel.

                // first create a scope to ensure the thread terminates
                let res = thread::scope(|scope| {
                    // start sorting the left side in another thread
                    let handle = scope.spawn(|_| {
                        self.sort(left)
                    });

                    // sort the right side in this thread
                    let right = self.sort(right);

                    // wait for the left side to finish
                    let left = handle.join().unwrap();

                    (left, right)
                }).unwrap();

                self.available_threads.fetch_add(1, Ordering::Release);

                res
            } else {
                // otherwise spawn both in this thread
                let left = self.sort(left);
                let right = self.sort(right);

                (left, right)
            };


            Self::merge(&left, &right)
        }
    }
}

impl ParallelMergeSorter {
    fn merge<T>(mut left: &[T], mut right: &[T]) -> Vec<T> where T: PartialOrd + Clone {
        let mut result: Vec<T> = Vec::with_capacity(right.len() + left.len());

        while !left.is_empty() && !right.is_empty() {
            let smallest = unsafe {
                if left.get_unchecked(0).clone() <= right.get_unchecked(0).clone() {
                    let (first, rest) = left.split_first().unwrap();
                    left = rest;
                    first
                } else {
                    let (first, rest) = right.split_first().unwrap();
                    right = rest;
                    first
                }
            };

            result.push(smallest.to_owned());
        }

        // consume remaining elements
        result.extend_from_slice(left);
        result.extend_from_slice(right);

        result
    }
}

#[cfg(test)]
#[quickcheck]
fn test_parallel_merge_sort(arr: Vec<TestNum>) {
    let sorter = ParallelMergeSorter::default();
    check_out_of_place_sorter(&sorter, arr)
}