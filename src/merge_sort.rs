use crate::OutOfPlaceSorterSorter;
#[cfg(test)]
use crate::test_utils::{check_out_of_place_sorter, TestNum};

pub struct MergeSorter;

impl<T> OutOfPlaceSorterSorter<T> for MergeSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &[T]) -> Vec<T> {
        if array.len() <= 1 {
            array.to_vec()
        } else {
            let middle = array.len() / 2;
            let (left, right) = array.split_at(middle);

            let left = self.sort(left);
            let right = self.sort(right);

            Self::merge(&left, &right)
        }
    }
}

impl MergeSorter {
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
fn test_merge_sort(arr: Vec<TestNum>) {
    let sorter = MergeSorter;
    check_out_of_place_sorter(&sorter, arr)
}