use arrow2::compute::merge_sort::SortOptions;
use arrow2::compute::sort;
use criterion::{criterion_group, criterion_main, Criterion};

use arrow2::array::*;
use arrow2::util::bench_util::*;
use arrow2::{compute::aggregate::*, datatypes::DataType};

fn bench_sum(arr_a: &PrimitiveArray<f32>) {
    sum(criterion::black_box(arr_a)).unwrap();
}

fn bench_min(arr_a: &PrimitiveArray<f32>) {
    min_primitive(criterion::black_box(arr_a)).unwrap();
}

fn bench_sort_limit(arr_a: &PrimitiveArray<f32>) {
    let opt = SortOptions {
        descending: true,
        nulls_first: false,
    };
    let _: PrimitiveArray<i32> =
        sort::sort_to_indices(criterion::black_box(arr_a), &opt, Some(100)).unwrap();
}

fn add_benchmark(c: &mut Criterion) {
    (10..=20).step_by(1).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);
        let arr_a = create_primitive_array::<f32>(size, DataType::Float32, 0.0);

        c.bench_function(&format!("arrow2-sum 2^{} f32", log2_size), |b| {
            b.iter(|| bench_sum(&arr_a))
        });
        c.bench_function(&format!("arrow2-min 2^{} f32", log2_size), |b| {
            b.iter(|| bench_min(&arr_a))
        });

        c.bench_function(&format!("arrow2-sort 2^{} f32", log2_size), |b| {
            b.iter(|| bench_sort_limit(&arr_a))
        });

        let arr_a = create_primitive_array::<f32>(size, DataType::Float32, 0.1);

        c.bench_function(&format!("arrow2-sum null 2^{} f32", log2_size), |b| {
            b.iter(|| bench_sum(&arr_a))
        });

        c.bench_function(&format!("arrow2-min null 2^{} f32", log2_size), |b| {
            b.iter(|| bench_min(&arr_a))
        });
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
