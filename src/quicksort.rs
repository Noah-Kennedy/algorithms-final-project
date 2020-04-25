use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct QuickSorter;

impl<T> InPlaceSorter<T> for QuickSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        if array.len() > 1 {
            quicksort(array, 0, array.len() - 1);
        }
    }
}

fn quicksort<T>(array: &mut [T], low: usize, high: usize) where T: PartialOrd + Clone {
    if low < high {
        let p = partition(array, low, high);
        quicksort(array, low, p);
        quicksort(array, p + 1, high);
    }
}

fn partition<T>(array: &mut [T], low: usize, high: usize) -> usize where T: PartialOrd + Clone {
    let pivot = array[(high + low) / 2].clone();
    let mut i = low as isize - 1;
    let mut j = high + 1;

    loop {
        unsafe {
            while {
                i += 1;
                array.get_unchecked(i as usize).clone() < pivot
            } {}
            while {
                j -= 1;
                array.get_unchecked(j).clone() > pivot
            } {}
            if i as usize >= j {
                return j;
            }

            array.swap(i as usize, j);
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_quick_sort(arr: Vec<TestNum>) {
    let sorter = QuickSorter;
    check_in_place_sorter(&sorter, arr)
}