use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jaggedarray::jagged_array::{JaggedArray, JaggedArrayViewTrait};
use num::{cast::AsPrimitive, traits::{ConstOne, ConstZero, NumAssignOps}, Num};
fn nested_2d_vector_iteration(data: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for i in data {
        for j in i {
            result += j;
        }
    }
    result
}

fn nested_3d_vector_iteration(data: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut result = 0;
    for i in data {
        for j in i {
            for k in j {
                result += k;
            }
        }
    }
    result
}

fn get_2d_nested_vector(iteration: usize) -> Vec<Vec<usize>> {
    (0..iteration)
        .map(|x| {
            let a = (0..x + 1).collect::<Vec<usize>>();
            let a = a.leak(); // simulate other allocations in real workload
            a.to_vec()
        })
        .collect::<Vec<_>>()
}

fn get_3d_nested_vector(iteration: usize) -> Vec<Vec<Vec<usize>>> {
    (0..iteration)
        .map(|x| {
            (0..x + 1)
                .map(|y| {
                    let a = (0..y + 1).collect::<Vec<usize>>();
                    let a = a.leak(); // simulate other allocations in real workload
                    a.to_vec()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn nested_2d_jagged_array_iteration<T: AsPrimitive<usize> + Num+ConstOne+ConstZero>(
    data: &JaggedArray<usize, Vec<T>, 2>,
) -> usize {
    let mut result = 0;
    unsafe {
        let len = data.len();
        for i in 0..len {
            let view = data.view_unchecked::<1, 1>([i]);
            for j in 0..view.len() {
                result += view.get_unchecked([j]);
            }
        }
    }
    result
}

fn nested_3d_jagged_array_iteration<T: AsPrimitive<usize> + Num+ConstOne+ConstZero>(
    data: &JaggedArray<usize, Vec<T>, 3>,
) -> usize {
    let mut result = 0;
    unsafe {
        let dims = data.len();
        for i in 0..dims {
            let view = data.view_unchecked::<1, 2>([i]);
            for j in 0..view.len() {
                let view = view.view_unchecked::<1, 1>([j]);
                for k in 0..view.len() {
                    result += view.get_unchecked([k]);
                }
            }
        }
    }
    result
}

fn get_2d_jagged_array<T: AsPrimitive<usize> + Num + NumAssignOps + std::cmp::PartialOrd+ConstOne+ConstZero>(
    iteration: usize,
) -> JaggedArray<usize, Vec<T>, 2> where usize: AsPrimitive<T> {
    let mut a = JaggedArray::<usize, Vec<T>, 2>::new();
    for i in 0..iteration {
        a.new_row::<0>();
        for j in 0..i + 1 {
            a.push_to_last_row(j);
        }
    }
    a
}

fn get_3d_jagged_array<T: AsPrimitive<usize> + Num + NumAssignOps + std::cmp::PartialOrd+ConstOne+ConstZero>(
    iteration: usize,
) -> JaggedArray<usize, Vec<T>, 3> where usize: AsPrimitive<T> {
    let mut a = JaggedArray::<usize, Vec<T>, 3>::new();
    for i in 0..iteration {
        a.new_row::<0>();
        for j in 0..i + 1 {
            a.new_row::<1>();
            for k in 0..j + 1 {
                a.push_to_last_row(k);
            }
        }
    }
    a
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.sample_size(300).measurement_time(Duration::from_secs(8));
    let a = get_2d_nested_vector(10);
    group.bench_function("2d nested vector iteration 10", |b| {
        b.iter(|| nested_2d_vector_iteration(black_box(&a)))
    });
    let a = get_2d_jagged_array::<usize>(10);
    group.bench_function("2d jagged array iteration 10", |b| {
        b.iter(|| nested_2d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_2d_nested_vector(100);
    group.bench_function("2d nested vector iteration 100", |b| {
        b.iter(|| nested_2d_vector_iteration(black_box(&a)))
    });
    let a = get_2d_jagged_array::<usize>(100);
    group.bench_function("2d jagged array iteration 100", |b| {
        b.iter(|| nested_2d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_2d_nested_vector(1000);
    group.bench_function("2d nested vector iteration 1000", |b| {
        b.iter(|| nested_2d_vector_iteration(black_box(&a)))
    });
    let a = get_2d_jagged_array::<usize>(1000);
    group.bench_function("2d jagged array iteration 1000", |b| {
        b.iter(|| nested_2d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_2d_jagged_array::<u32>(1000);
    group.bench_function("2d jagged array iteration 1000(U32)", |b| {
        b.iter(|| nested_2d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_2d_nested_vector(10000);
    group.bench_function("2d nested vector iteration 10000", |b| {
        b.iter(|| nested_2d_vector_iteration(black_box(&a)))
    });
    let a = get_2d_jagged_array::<usize>(10000);
    group.bench_function("2d jagged array iteration 10000", |b| {
        b.iter(|| nested_2d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_2d_jagged_array::<u32>(10000);
    group.bench_function("2d jagged array iteration 10000(U32)", |b| {
        b.iter(|| nested_2d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_nested_vector(10);
    group.bench_function("3d nested vector iteration 10", |b| {
        b.iter(|| nested_3d_vector_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<usize>(10);
    group.bench_function("3d jagged array iteration 10", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<u16>(10);
    group.bench_function("3d jagged array iteration 10(U16)", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_nested_vector(33);
    group.bench_function("3d nested vector iteration 33", |b| {
        b.iter(|| nested_3d_vector_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<usize>(33);
    group.bench_function("3d jagged array iteration 33", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<u16>(33);
    group.bench_function("3d jagged array iteration 33(U16)", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_nested_vector(100);
    group.bench_function("3d nested vector iteration 100", |b| {
        b.iter(|| nested_3d_vector_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<u64>(100);
    group.bench_function("3d jagged array iteration 100", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<u32>(100);
    group.bench_function("3d jagged array iteration 100(U32)", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_nested_vector(500);
    group.bench_function("3d nested vector iteration 500", |b| {
        b.iter(|| nested_3d_vector_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<u32>(500);
    group.bench_function("3d jagged array iteration 500", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
    let a = get_3d_jagged_array::<u32>(500);
    group.bench_function("3d jagged array iteration 500(U32)", |b| {
        b.iter(|| nested_3d_jagged_array_iteration(black_box(&a)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
