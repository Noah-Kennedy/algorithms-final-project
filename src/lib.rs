#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod bubble_sort;
pub mod cocktail_shaker_sort;
pub mod selection_sort;
pub mod bingo_sort;
pub mod insertion_sort;
pub mod binary_insertion_sort;
pub mod merge_sort;
pub mod heapsort;
pub mod quicksort;

#[cfg(test)]
pub mod test_utils;

#[cfg(feature = "benchmark-internals")]
pub const SIZES: [usize; 11] = [0, 1_000, 2_000, 3_000, 4_000, 5_000, 6_000, 7_000, 8_000, 9_000, 10_000];

pub trait OutOfPlaceSorterSorter<T> {
    fn sort(&self, array: &[T]) -> Vec<T>;
}

pub trait InPlaceSorter<T> {
    fn sort(&self, array: &mut [T]);
}