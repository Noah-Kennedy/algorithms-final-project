use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct BingoSorter;

impl<T> InPlaceSorter<T> for BingoSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        if array.len() > 1 {
            let mut max = array.len() - 1;

            unsafe {
                let mut next = array.get_unchecked(max).clone();

                for i in (0..max).rev() {
                    if array[i] > next {
                        next = array.get_unchecked(i).clone()
                    }
                }

                while (max > 0) && (array.get_unchecked(max).clone() == next) {
                    max -= 1;
                }

                while max > 0 {
                    let value = next;
                    next = array.get_unchecked(max).clone();

                    for i in (0..max).rev() {
                        if array.get_unchecked(i).clone() == value {
                            array.swap(i, max);
                            max -= 1;
                        } else if array.get_unchecked(i).clone() > next {
                            next = array.get_unchecked(i).clone()
                        }
                    }

                    while (max > 0) && (array.get_unchecked(max).clone() == next) {
                        max -= 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_bubble_sort(arr: Vec<TestNum>) {
    let sorter = BingoSorter;
    check_in_place_sorter(&sorter, arr)
}