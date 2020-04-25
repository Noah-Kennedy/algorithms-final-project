use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct BinaryInsertionSorter;

impl<T> InPlaceSorter<T> for BinaryInsertionSorter where T: Ord + Clone {
    fn sort(&self, array: &mut [T]) {
        for i in 1..array.len() {
            unsafe {
                let value = array.get_unchecked(i).clone();
                let mut j = i;

                let pos = array.get_unchecked(..i).binary_search(&value).unwrap_or_else(|pos| pos);

                while j > pos {
                    *array.get_unchecked_mut(j) = array.get_unchecked_mut(j - 1).clone();
                    j -= 1;
                }
                *array.get_unchecked_mut(j) = value;
            }
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_insertion_sort(arr: Vec<TestNum>) {
    let sorter = BinaryInsertionSorter;
    check_in_place_sorter(&sorter, arr)
}