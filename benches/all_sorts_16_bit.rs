use criterion::{AxisScale, BenchmarkId, Criterion, criterion_group, criterion_main, PlotConfiguration, Throughput};
use jemallocator::Jemalloc;
use rand::seq::SliceRandom;
use rand::thread_rng;

use sorts_rs::{InPlaceSorter, OutOfPlaceSorterSorter};
use sorts_rs::binary_insertion_sort::BinaryInsertionSorter;
use sorts_rs::bubble_sort::BubbleSorter;
use sorts_rs::heapsort::HeapSorter;
use sorts_rs::insertion_sort::InsertionSorter;
use sorts_rs::merge_sort::MergeSorter;
use sorts_rs::quicksort::QuickSorter;

//#[cfg(not(target_env = "msvc"))]
//#[global_allocator]
//static GLOBAL: Jemalloc = Jemalloc;

type Num = u16;

const SIZES: [Num; 11] = [0, 1_000, 2_000, 3_000, 4_000, 5_000, 6_000, 7_000, 8_000, 9_000, 10_000];

fn bench_sorts(c: &mut Criterion) {
    let mut rng = thread_rng();
    let randoms: Vec<Vec<Num>> = SIZES
        .iter()
        .map(|&size| {
            let mut v: Vec<Num> = (0..size).collect();

            v.shuffle(&mut rng);

            v
        })
        .collect();
    let mut group = c.benchmark_group("Sorts 16");

    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for data in randoms.iter() {
        group.throughput(Throughput::Elements(data.len() as u64));
        let data: Vec<Num> = data.to_owned();
        group.bench_with_input(BenchmarkId::new("Bubble", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = BubbleSorter;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("InsertionSort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = InsertionSorter;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("BinaryInsertionSort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = BinaryInsertionSorter;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("MergeSort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = MergeSorter;
            sorter.sort(&mut data)
        }));
        group.bench_with_input(BenchmarkId::new("HeapSort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = HeapSorter;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("QuickSort", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            let sorter = QuickSorter;
            sorter.sort(&mut data);
            data
        }));
        group.bench_with_input(BenchmarkId::new("Std Stable (TimSort)", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            data.sort();
            data
        }));
        group.bench_with_input(BenchmarkId::new("Std Unstable (PDQSort)", data.len()), &data, |b, data| b.iter(|| {
            let mut data = data.clone();

            data.sort_unstable();
            data
        }));
    }

    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);