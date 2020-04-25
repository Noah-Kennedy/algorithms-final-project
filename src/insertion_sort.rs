use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct InsertionSorter;

impl<T> InPlaceSorter<T> for InsertionSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        for i in 1..array.len() {
            unsafe {
                let value = array.get_unchecked(i).clone();
                let mut j = i as isize - 1;

                while j >= 0 && array.get_unchecked(j as usize).clone() > value {
                    *array.get_unchecked_mut((j + 1) as usize) = array.get_unchecked(j as usize).clone();
                    j -= 1;
                }

                *array.get_unchecked_mut((j + 1) as usize) = value;
            }
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_insertion_sort(arr: Vec<TestNum>) {
    let sorter = InsertionSorter;
    check_in_place_sorter(&sorter, arr)
}