use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use sorts_rs::InPlaceSorter;
use sorts_rs::insertion_sort::InsertionSorter;
use sorts_rs::msd_radix_sort::MSDRadix;

fn random_array(size: usize, rng: &mut ThreadRng) -> Vec<u16> {
    let mut numbers = Vec::with_capacity(size);

    for _ in 0..size {
        numbers.push(rng.gen_range(0, 1000));
    }

    numbers
}

fn main() {
    let mut rng = thread_rng();
    let mut v: Vec<u16> = random_array(1000, &mut rng);

    println!("{:?}", v);

    let sorter = MSDRadix;
    sorter.sort(&mut v);

    println!("{:?}", v);
}