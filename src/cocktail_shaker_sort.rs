use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct CocktailShakerSorter;

impl<T> InPlaceSorter<T> for CocktailShakerSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        if array.len() > 1 {
            let mut swapped = true;
            let mut start = 0;
            let mut end = array.len() - 1;

            while swapped {
                swapped = false;

                unsafe {
                    for i in start..end {
                        if array.get_unchecked(i) > array.get_unchecked(i + 1) {
                            array.swap(i, i + 1);
                            swapped = true;
                        }
                    }

                    if !swapped {
                        break;
                    }

                    swapped = false;

                    end -= 1;

                    for i in (start..end).rev() {
                        if array.get_unchecked(i) > array.get_unchecked(i + 1) {
                            array.swap(i, i + 1);
                            swapped = true;
                        }
                    }

                    start += 1;
                }
            }
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_bubble_sort(arr: Vec<TestNum>) {
    let sorter = CocktailShakerSorter;
    check_in_place_sorter(&sorter, arr)
}