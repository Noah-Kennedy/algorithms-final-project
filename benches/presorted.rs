use criterion::{AxisScale, BenchmarkId, Criterion, criterion_group, criterion_main, PlotConfiguration, Throughput};
use jemallocator::Jemalloc;
use rand::seq::SliceRandom;
use rand::thread_rng;

use sorts_rs::{InPlaceSorter, OutOfPlaceSorterSorter};
use sorts_rs::binary_insertion_sort::BinaryInsertionSorter;
use sorts_rs::bingo_sort::BingoSorter;
use sorts_rs::bubble_sort::BubbleSorter;
use sorts_rs::cocktail_shaker_sort::CocktailShakerSorter;
use sorts_rs::heapsort::HeapSorter;
use sorts_rs::insertion_sort::InsertionSorter;
use sorts_rs::merge_sort::MergeSorter;
use sorts_rs::quicksort::QuickSorter;
use sorts_rs::selection_sort::SelectionSorter;

//#[cfg(not(target_env = "msvc"))]
//#[global_allocator]
//static GLOBAL: Jemalloc = Jemalloc;

type Num = u32;

const SIZES: [usize; 11] = [0, 1_000, 2_000, 3_000, 4_000, 5_000, 6_000, 7_000, 8_000, 9_000, 10_000];

fn make_lists() -> Vec<Vec<Num>> {
    let mut rng = thread_rng();

    SIZES.iter()
         .map(|&size| {
             let mut v: Vec<Num> = (0..100).cycle().take(size).collect();

             v.sort();

             v
         })
         .collect()
}

fn bench_n_2_sorts(c: &mut Criterion) {
    let randoms: Vec<Vec<Num>> = make_lists();
    let mut n2_sorts = c.benchmark_group("O(n^2) Presorted Sorts");

    n2_sorts.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));

    for data in randoms.iter() {
        n2_sorts.throughput(Throughput::Elements(data.len() as u64));
        let data: Vec<Num> = data.to_owned();
        n2_sorts.bench_with_input(BenchmarkId::new("Bubble Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = BubbleSorter;
            sorter.sort(&mut data);
            data
        }));
        n2_sorts.bench_with_input(BenchmarkId::new("Cocktail Shaker Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = CocktailShakerSorter;
            sorter.sort(&mut data);
            data
        }));
        n2_sorts.bench_with_input(BenchmarkId::new("Selection Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = SelectionSorter;
            sorter.sort(&mut data);
            data
        }));
        n2_sorts.bench_with_input(BenchmarkId::new("Bingo Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = BingoSorter;
            sorter.sort(&mut data);
            data
        }));
        n2_sorts.bench_with_input(BenchmarkId::new("Insertion Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = InsertionSorter;
            sorter.sort(&mut data);
            data
        }));
        n2_sorts.bench_with_input(BenchmarkId::new("Binary Insertion Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = BinaryInsertionSorter;
            sorter.sort(&mut data);
            data
        }));
    }

    n2_sorts.finish();
}

fn bench_n_log_n_sorts(c: &mut Criterion) {
    let randoms: Vec<Vec<Num>> = make_lists();

    let mut n_log_n_sorts = c.benchmark_group("O(nlogn) Presorted Sorts");
    n_log_n_sorts.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));

    for data in randoms.iter() {
        n_log_n_sorts.throughput(Throughput::Elements(data.len() as u64));
        let data: Vec<Num> = data.to_owned();
        n_log_n_sorts.bench_with_input(BenchmarkId::new("Bingo Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = BingoSorter;
            sorter.sort(&mut data);
            data
        }));
        n_log_n_sorts.bench_with_input(BenchmarkId::new("Merge Sort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = MergeSorter;
            sorter.sort(&mut data)
        }));
        n_log_n_sorts.bench_with_input(BenchmarkId::new("HeapSort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = HeapSorter;
            sorter.sort(&mut data);
            data
        }));
        n_log_n_sorts.bench_with_input(BenchmarkId::new("QuickSort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = QuickSorter;
            sorter.sort(&mut data);
            data
        }));
        n_log_n_sorts.bench_with_input(BenchmarkId::new("Std Stable Sort (TimSort)", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            data.sort();
            data
        }));
        n_log_n_sorts.bench_with_input(BenchmarkId::new("Std Unstable Sort (PDQSort)", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            data.sort_unstable();
            data
        }));
    }

    n_log_n_sorts.finish();
}

criterion_group!(benches, bench_n_2_sorts, bench_n_log_n_sorts);
criterion_main!(benches);