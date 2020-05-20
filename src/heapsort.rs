use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};
use std::mem::MaybeUninit;
use std::mem;

pub struct HeapSorter;

impl<T> InPlaceSorter<T> for HeapSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        if array.len() > 1 {
            Self::heapify(array);

            for i in (1..(array.len())).rev() {
                array.swap(0, i);

                Self::sift_down(array, 0, i - 1);
            }
        }
    }
}

impl HeapSorter {
    fn heapify<T>(array: &mut [T]) where T: PartialOrd + Clone {
        let n = array.len() - 1;

        for i in (0..=Self::parent(n)).rev() {
            Self::sift_down(array, i, n);
        }
    }

    fn sift_down<T>(array: &mut [T], start: usize, end: usize) where T: PartialOrd + Clone {
        let mut root = start;

        while Self::left(root) <= end {
            let child = Self::left(root);
            let adjacent = child + 1;
            let mut swap = root;

            unsafe {
                if array.get_unchecked(swap).clone() < array.get_unchecked(child).clone() {
                    swap = child;
                }

                if adjacent <= end && array.get_unchecked(swap).clone() < array.get_unchecked(adjacent).clone() {
                    swap = adjacent;
                }
            }

            if swap == root {
                return;
            } else {
                array.swap(root, swap);
                root = swap;
            }
        }
    }

    #[inline]
    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    #[inline]
    fn left(index: usize) -> usize {
        2 * index + 1
    }
}

#[cfg(test)]
#[quickcheck]
fn test_heap_sort(arr: Vec<TestNum>) {
    let sorter = HeapSorter;
    check_in_place_sorter(&sorter, arr)
}