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
use arrow2::buffer::MutableBuffer;
use criterion::Criterion;

use arrow2::array::*;
use arrow2::compute::cast;
use arrow2::datatypes::*;
use arrow2::util::bench_util::*;
use num_traits::AsPrimitive;

fn add_benchmark(c: &mut Criterion) {
    let size = 1048576;
    let u32_array = create_primitive_array::<u32>(size, DataType::UInt32, 0.1);

    c.bench_function("cast u32 to u64 1048576", |b| {
        b.iter(|| cast_array(&u32_array, DataType::UInt64))
    });

    c.bench_function("cast u32 to u64 v2 1048576", |b| {
        b.iter(|| cast_u32_u64(&u32_array))
    });
}

fn cast_array(array: &dyn Array, to_type: DataType) {
    criterion::black_box(cast::cast(array, &to_type).unwrap());
}

fn cast_u32_u64(array: &UInt32Array) {
    criterion::black_box({
        let it = array
            .values()
            .as_slice()
            .iter()
            .map(|c| AsPrimitive::<u64>::as_(*c));

        let buffer = unsafe { MutableBuffer::from_trusted_len_iter_unchecked(it) };
        UInt64Array::from_data(DataType::UInt64, buffer.into(), array.validity().cloned());
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
