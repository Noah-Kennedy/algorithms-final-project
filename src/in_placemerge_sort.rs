use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct InPlaceMergeSorter;

impl<T> InPlaceSorter<T> for InPlaceMergeSorter where T: PartialOrd + Clone {
    fn sort(&self, array: &mut [T]) {
        if array.len() > 1 {
            let middle = array.len() / 2;
            let (left, right) = array.split_at(middle);

            let mut left = left.to_owned();
            let mut right = right.to_owned();

            self.sort(&mut left);
            self.sort(&mut right);

            Self::merge(array, &left, &right)
        }
    }
}

impl InPlaceMergeSorter {
    fn merge<T>(array: &mut [T], mut left: &[T], mut right: &[T]) where T: PartialOrd + Clone {
        assert_eq!(array.len(), left.len() + right.len());

        let mut next_elem = 0;

        while !left.is_empty() && !right.is_empty() {
            let smallest = unsafe {
                if left.get_unchecked(0).clone() <= right.get_unchecked(0).clone() {
                    let (first, rest) = left.split_first().unwrap();
                    left = rest;
                    first
                } else {
                    let (first, rest) = right.split_first().unwrap();
                    right = rest;
                    first
                }
            };

            array[next_elem] = smallest.to_owned();
            next_elem += 1;
        }

        for elem in left {
            array[next_elem] = elem.clone();
            next_elem += 1;
        }

        for elem in right {
            array[next_elem] = elem.clone();
            next_elem += 1;
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_merge_sort(arr: Vec<TestNum>) {
    let sorter = InPlaceMergeSorter;
    check_in_place_sorter(&sorter, arr)
}