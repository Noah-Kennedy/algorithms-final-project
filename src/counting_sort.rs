use crate::InPlaceSorter;
//#[cfg(test)]
//use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct CountingSorter256;

impl InPlaceSorter<u8> for CountingSorter256 where {
    fn sort(&self, array: &mut [u8]) {
        if array.len() > 1 {
            let mut output = vec![0; array.len()];
            let mut count = [0; 256];

            for i in array.iter() {
                count[*i as usize] += 1;
            }

            for i in 1..256 {
                count[i] += count[i - 1];
            }

            for i in (0..array.len()).rev() {
                let offset = (array[i]) as usize;
                output[count[array[i] as usize] - 1] = array[i];
                count[offset] -= 1;
            }

            array.copy_from_slice(&output[0..array.len()])
        }
    }
}

//#[cfg(test)]
//#[quickcheck]
//fn test_counting_sort(arr: Vec<TestNum>) {
//    let sorter = CountingSorter256;
//    check_in_place_sorter(&sorter, arr)
//}