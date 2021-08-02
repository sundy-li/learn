// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[macro_use]
extern crate criterion;
use criterion::Criterion;

use arrow::compute::kernels::aggregate::*;
use arrow::util::bench_util::*;
use arrow::{array::*, datatypes::Float32Type};

fn bench_sum(arr_a: &Float32Array) {
    sum(criterion::black_box(arr_a)).unwrap();
}

fn bench_min(arr_a: &Float32Array) {
    min(criterion::black_box(arr_a)).unwrap();
}

fn bench_max(arr_a: &Float32Array) {
    max(criterion::black_box(arr_a)).unwrap();
}

fn bench_min_string(arr_a: &StringArray) {
    min_string(criterion::black_box(&arr_a)).unwrap();
}

fn add_benchmark(c: &mut Criterion) {
    (10..=20).step_by(1).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);
        let arr_a = create_primitive_array::<Float32Type>(size, 0.0);

        c.bench_function(&format!("arrow1-sum 2^{} f32", log2_size), |b| {
            b.iter(|| bench_sum(&arr_a))
        });
        c.bench_function(&format!("arrow1-min 2^{} f32", log2_size), |b| {
            b.iter(|| bench_min(&arr_a))
        });
        c.bench_function(&format!("arrow1-max 2^{} f32", log2_size), |b| {
            b.iter(|| bench_max(&arr_a))
        });

        let arr_b = create_string_array::<i32>(size, 0.0);
        c.bench_function(&format!("arrow2-min string 2^{} f32", log2_size), |b| {
            b.iter(|| bench_min_string(&arr_b))
        });

        let arr_b = create_string_array::<i32>(size, 0.5);
        c.bench_function(
            &format!("arrow2-min nulls string 2^{} f32", log2_size),
            |b| b.iter(|| bench_min_string(&arr_b)),
        );
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
