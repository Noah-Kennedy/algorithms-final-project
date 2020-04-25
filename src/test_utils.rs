use crate::{InPlaceSorter, OutOfPlaceSorterSorter};

#[cfg(test)]
pub type TestNum = u64;

#[cfg(test)]
pub fn check_out_of_place_sorter(sorter: &dyn OutOfPlaceSorterSorter<TestNum>, arr: Vec<TestNum>) {
    let mut expected = arr.clone();

    expected.sort();
    let actual = sorter.sort(&arr);

    assert_eq!(expected, actual)
}

#[cfg(test)]
pub fn check_in_place_sorter(sorter: &dyn InPlaceSorter<TestNum>, arr: Vec<TestNum>) {
    let mut expected = arr.clone();
    let mut actual = arr;

    expected.sort();
    sorter.sort(actual.as_mut());

    assert_eq!(expected, actual)
}