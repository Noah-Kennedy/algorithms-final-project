use crate::InPlaceSorter;
#[cfg(test)]
use crate::test_utils::{check_in_place_sorter, TestNum};

pub struct MSDRadix;

impl InPlaceSorter<u16> for MSDRadix where {
    fn sort(&self, array: &mut [u16]) {
        if array.len() > 1 {
            let mut exponent: u32 = 1;
            let mut out = vec![0; array.len()];

            for i in 0..2 {
                let mut buckets = [0; 256];

                // for each element, we do a
                for &elem in array.iter() {
                    buckets[(elem as u32 / exponent % 256) as usize] += 1;
                }

                for j in 1..256 {
                    buckets[j] += buckets[j - 1];
                }

                for &j in array.iter().rev() {
                    let v = (j as u32 / exponent % 256) as usize;
                    buckets[v] -= 1;

                    out[buckets[v]] = j;
                }

                array.copy_from_slice(&out);

                exponent *= 256
            }
        }
    }
}

#[cfg(test)]
#[quickcheck]
fn test_radix_sort(arr: Vec<TestNum>) {
    let sorter = MSDRadix;
    check_in_place_sorter(&sorter, arr)
}