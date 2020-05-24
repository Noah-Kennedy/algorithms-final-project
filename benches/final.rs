use criterion::{AxisScale, BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, PlotConfiguration, Throughput};
use criterion::measurement::WallTime;
use rand::prelude::ThreadRng;
use rand::Rng;
use rand::seq::SliceRandom;

use sorts_rs::{InPlaceSorter, OutOfPlaceSorterSorter};
use sorts_rs::binary_insertion_sort::BinaryInsertionSorter;
use sorts_rs::bingo_sort::BingoSorter;
use sorts_rs::bubble_sort::BubbleSorter;
use sorts_rs::cocktail_shaker_sort::CocktailShakerSorter;
use sorts_rs::heapsort::HeapSorter;
use sorts_rs::insertion_sort::InsertionSorter;
use sorts_rs::merge_sort::MergeSorter;
use sorts_rs::msd_radix_sort::MSDRadix;
use sorts_rs::parallel_merge_sort::ParallelMergeSorter;
use sorts_rs::quicksort::QuickSorter;
use sorts_rs::selection_sort::SelectionSorter;
use sorts_rs::counting_sort::CountingSorter1024;

// faster allocator
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

type Num = u16;

const SIZES: [usize; 10] = [1, 50, 100, 500, 1_000, 5_000, 10_000, 100_000, 500_000, 1_000_000];

// always a good answer when you dont know the question
const SAMPLE_SIZE: usize = 42;

fn make_random_lists<F>(order_modifier: F) -> Vec<Vec<Num>> where F: Fn(Vec<Num>, &mut ThreadRng) -> Vec<Num> {
    let mut rng = rand::thread_rng();

    SIZES.iter()
         .map(|&size| {
             let v: Vec<Num> = random_array(size, &mut rng);

             order_modifier(v, &mut rng)
         })
         .collect()
}

fn random_array(size: usize, rng: &mut ThreadRng) -> Vec<Num> {
    let mut numbers = Vec::with_capacity(size);

    for _ in 0..size {
        numbers.push(rng.gen_range(0, 1_024));
    }

    numbers
}

fn partially_sorted(c: &mut Criterion) {
    let randoms = make_random_lists(|mut v, _| {
        let mut chunk_size = v.len() / 20;

        if chunk_size > 1 {
            v.chunks_mut(chunk_size).for_each(|c| {
                c.sort()
            });
        }

        v
    });
    let mut group = c.benchmark_group("Sorts - Presorted Runs");

    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));
    group.sample_size(SAMPLE_SIZE);

    run_benchmarks(&mut group, randoms);

    group.finish();
}

fn unsorted(c: &mut Criterion) {
    let randoms = make_random_lists(|mut v, rng| {
        v.shuffle(rng);

        v
    });

    let mut group = c.benchmark_group("Sorts - Unsorted Data");

    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));
    group.sample_size(SAMPLE_SIZE);

    run_benchmarks(&mut group, randoms);

    group.finish();
}

fn run_benchmarks(group: &mut BenchmarkGroup<WallTime>, randoms: Vec<Vec<Num>>) {
    for data in randoms.iter() {
        group.throughput(Throughput::Elements(data.len() as u64));
        let data: Vec<Num> = data.to_owned();
        //group.bench_with_input(BenchmarkId::new("Bubble Sort", data.len()), &data, |b, data| b.iter(|| {
        //    let mut data = data.clone();
        //    let sorter = BubbleSorter;
        //    sorter.sort(&mut data);
        //    data
        //}));
        //group.bench_with_input(BenchmarkId::new("Cocktail Shaker Sort", data.len()), &data, |b, data| b.iter(|| {
        //    let mut data = data.clone();
        //    let sorter = CocktailShakerSorter;
        //    sorter.sort(&mut data);
        //    data
        //}));
        //group.bench_with_input(BenchmarkId::new("Selection Sort", data.len()), &data, |b, data| b.iter(|| {
        //    let mut data = data.clone();
        //    let sorter = SelectionSorter;
        //    sorter.sort(&mut data);
        //    data
        //}));

        //group.bench_with_input(BenchmarkId::new("Insertion Sort", data.len()), &data, |b, data| b.iter(|| {
        //    let mut data = data.clone();
        //    let sorter = InsertionSorter;
        //    sorter.sort(&mut data);
        //    data
        //}));
        //group.bench_with_input(BenchmarkId::new("Binary Insertion Sort", data.len()), &data, |b, data| b.iter(|| {
        //    let mut data = data.clone();
        //    let sorter = BinaryInsertionSorter;
        //    sorter.sort(&mut data);
        //    data
        //}));
        group.bench_with_input(BenchmarkId::new("Radix Sort", data.len()), &data, |b, data| b.iter
        (|| {
            let mut data = data.clone();
            let sorter = MSDRadix;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("Counting Sort", data.len()), &data, |b, data| b
            .iter
        (|| {
            let mut data = data.clone();
            let sorter = CountingSorter1024;
            sorter.sort(&mut data);
            data
        }));
        //group.bench_with_input(BenchmarkId::new("Bingo Sort", data.len()), &data, |b, data| b.iter(|| {
        //    let mut data = data.clone();
        //    let sorter = BingoSorter;
        //    sorter.sort(&mut data);
        //    data
        //}));
        group.bench_with_input(BenchmarkId::new("Merge Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();
            let sorter = MergeSorter;
            sorter.sort(&mut data)
        }));
        group.bench_with_input(BenchmarkId::new("Parallel Merge Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();
            let sorter = ParallelMergeSorter::default();
            sorter.sort(&mut data)
        }));
        group.bench_with_input(BenchmarkId::new("Heap Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();
            let sorter = HeapSorter;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("Quick Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();
            let sorter = QuickSorter;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("Std Stable Sort (Tim Sort)", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();
            data.sort();
            data
        }));
        group.bench_with_input(BenchmarkId::new("Std Unstable Sort (PDQ Sort)", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();
            data.sort_unstable();
            data
        }));
    }
}

criterion_group!(benches,partially_sorted, unsorted);
criterion_main!(benches);