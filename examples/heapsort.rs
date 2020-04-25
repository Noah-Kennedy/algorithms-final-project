use rand::seq::SliceRandom;
use rand::thread_rng;
use sorts_rs::heapsort::HeapSorter;
use sorts_rs::InPlaceSorter;

fn main() {
    let mut rng = thread_rng();
    let mut v: Vec<u64> = (0..10_000).collect();

    for _ in 0..100 {
        v.shuffle(&mut rng);

        let sorter = HeapSorter;

        sorter.sort(&mut v)
    }
}