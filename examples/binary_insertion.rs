use rand::seq::SliceRandom;
use rand::thread_rng;
use sorts_rs::InPlaceSorter;
use sorts_rs::binary_insertion_sort::BinaryInsertionSorter;

fn main() {
    let mut rng = thread_rng();
    let mut v: Vec<u64> = (0..10_000).collect();

    for _ in 0..100 {
        v.shuffle(&mut rng);

        let sorter = BinaryInsertionSorter;

        sorter.sort(&mut v)
    }
}