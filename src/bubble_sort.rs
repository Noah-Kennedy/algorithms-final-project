use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct BubbleSorter;

impl<T> InPlaceSorter<T> for BubbleSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        let mut n = array.len();

        while n != 0 {
            let mut new = 0;

            for i in 1..(n) {
                unsafe {
                    if array.get_unchecked(i - 1).clone() > array.get_unchecked(i).clone() {
                        array.swap(i, i - 1);
                        new = i;
                    }
                }
            }

            n = new;
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_bubble_sort(arr: Vec<TestNum>) {
    let sorter = BubbleSorter;
    check_in_place_sorter(&sorter, arr)
}