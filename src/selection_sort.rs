use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct SelectionSorter;

impl<T> InPlaceSorter<T> for SelectionSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        for i in 0..array.len() {
            let mut j_min = i;

            for j in (i + 1)..array.len() {
                unsafe {
                    if array.get_unchecked(j) < array.get_unchecked(j_min) {
                        j_min = j;
                    }
                }
            }

            if j_min != i {
                array.swap(i, j_min);
            }
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_bubble_sort(arr: Vec<TestNum>) {
    let sorter = SelectionSorter;
    check_in_place_sorter(&sorter, arr)
}