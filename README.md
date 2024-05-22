# JaggedArray

[![crates.io](https://img.shields.io/crates/v/jaggedarray)](https://crates.io/crates/jaggedarray)
[![docs.rs](https://docs.rs/jaggedarray/badge.svg)](https://docs.rs/jaggedarray)

This crate provides multidimensional jagged arrays. Most functionalities that you expect a jagged array to support (in an efficient manner) are supported.

## Features

- Multidimensional jagged arrays on a contiguous buffer.
- Methods for efficient array traversal, mutation, adding and removing last rows.
- Short index type(like `u8`,`u16`) support.

## Getting Started

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
jaggedarray = "0.1.0"
```

### Example

```rust
use jaggedarray::JaggedArray;
use jaggedarray::utils::U16;

let mut data = jagged_array::JaggedArray::<i32, U16, 3>::new();
data.new_row::<0>();
data.new_row::<1>();
data.push_to_last_row(1);
assert!(data[[0, 0, 0]] == 1);
assert!(data.view::<1, 2>([0]).view::<1, 1>([0])[[0]] == 1);
data.new_row::<1>();
data.push_to_last_row(4);
data.push_to_last_row(5);
assert!(data[[0, 1, 0]] == 4);
assert!(data[[0, 1, 1]] == 5);
data.new_row::<1>();
data.push_to_last_row(7);
data.push_to_last_row(8);
data.push_to_last_row(9);
assert!(data[[0, 2, 0]] == 7);
assert!(data[[0, 2, 1]] == 8);
assert!(data[[0, 2, 2]] == 9);
data.new_row::<0>();
data.new_row::<1>();
data.push_to_last_row(10);
data.push_to_last_row(11);
data.push_to_last_row(12);
data.push_to_last_row(13);
assert!(data[[1, 0, 0]] == 10);
assert!(data[[1, 0, 1]] == 11);
assert!(data[[1, 0, 2]] == 12);
assert!(data[[1, 0, 3]] == 13);
data.new_row::<0>();
data.new_row::<0>();
data.new_row::<1>();
data.push_to_last_row(100);
// assert!(data[[2, 0, 0]] == 100);
assert!(data[[3, 0, 0]] == 100);
data.append(data.clone());
assert!(data[[7, 0, 0]] == 100);
data.remove_last_row::<0>();
assert!(data[[0, 0, 0]] == 1);
assert!(data[[0, 1, 0]] == 4);
assert!(data[[0, 1, 1]] == 5);
assert!(data[[0, 2, 0]] == 7);
assert!(data[[0, 2, 1]] == 8);
assert!(data[[0, 2, 2]] == 9);
```

### Benchmark

The performance of nested vectors heavily depends on usage patterns and allocator behaviour. If you can allocate all your vectors at once and/or your vector is really small, then nested vectors could be more efficient. Otherwise, jagged arrays are more efficient. 

```txt
sample-size/2d nested vector iteration 1000
                        time:   [95.485 µs 96.837 µs 98.321 µs]
                        change: [+1.1201% +2.3958% +3.8359%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 46 outliers among 300 measurements (15.33%)
  14 (4.67%) high mild
  32 (10.67%) high severe
sample-size/2d jagged array iteration 1000
                        time:   [71.956 µs 72.113 µs 72.275 µs]
                        change: [-0.5941% +0.0530% +0.7736%] (p = 0.88 > 0.05)
                        No change in performance detected.
Found 13 outliers among 300 measurements (4.33%)
  8 (2.67%) high mild
  5 (1.67%) high severe
sample-size/2d jagged array iteration 1000(U32)
                        time:   [56.048 µs 56.211 µs 56.400 µs]
                        change: [-1.0558% -0.3365% +0.4911%] (p = 0.41 > 0.05)
                        No change in performance detected.
Found 13 outliers among 300 measurements (4.33%)
  8 (2.67%) high mild
  5 (1.67%) high severe
sample-size/2d nested vector iteration 10000
                        time:   [16.935 ms 16.976 ms 17.025 ms]
                        change: [-0.5051% -0.1652% +0.1817%] (p = 0.35 > 0.05)
                        No change in performance detected.
Found 16 outliers among 300 measurements (5.33%)
  9 (3.00%) high mild
  7 (2.33%) high severe
sample-size/2d jagged array iteration 10000
                        time:   [15.319 ms 15.348 ms 15.380 ms]
                        change: [-0.2571% +0.0457% +0.3378%] (p = 0.76 > 0.05)
                        No change in performance detected.
Found 10 outliers among 300 measurements (3.33%)
  8 (2.67%) high mild
  2 (0.67%) high severe
sample-size/2d jagged array iteration 10000(U32)
                        time:   [15.135 ms 15.166 ms 15.201 ms]
                        change: [-0.4291% -0.1197% +0.1793%] (p = 0.46 > 0.05)
                        No change in performance detected.
Found 10 outliers among 300 measurements (3.33%)
  7 (2.33%) high mild
  3 (1.00%) high severe
sample-size/3d nested vector iteration 10
                        time:   [96.544 ns 96.762 ns 96.996 ns]
                        change: [-1.1622% -0.5117% +0.2933%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 15 outliers among 300 measurements (5.00%)
  7 (2.33%) high mild
  8 (2.67%) high severe
sample-size/3d jagged array iteration 10
                        time:   [212.26 ns 212.67 ns 213.11 ns]
                        change: [-1.4673% -0.9200% -0.3818%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 300 measurements (3.67%)
  4 (1.33%) high mild
  7 (2.33%) high severe
sample-size/3d jagged array iteration 10(U16)
                        time:   [212.59 ns 213.00 ns 213.47 ns]
                        change: [-1.5934% -0.9125% -0.1971%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 19 outliers among 300 measurements (6.33%)
  8 (2.67%) high mild
  11 (3.67%) high severe
sample-size/3d nested vector iteration 100
                        time:   [38.712 µs 38.792 µs 38.874 µs]
                        change: [-2.2883% -1.7169% -1.1787%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 300 measurements (6.00%)
  3 (1.00%) low mild
  7 (2.33%) high mild
  8 (2.67%) high severe
sample-size/3d jagged array iteration 100
                        time:   [77.382 µs 77.567 µs 77.763 µs]
                        change: [-1.0918% -0.4742% +0.2046%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 18 outliers among 300 measurements (6.00%)
  8 (2.67%) high mild
  10 (3.33%) high severe
sample-size/3d jagged array iteration 100(U32)
                        time:   [57.242 µs 57.371 µs 57.516 µs]
                        change: [-0.7841% -0.3735% -0.0037%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 10 outliers among 300 measurements (3.33%)
  7 (2.33%) high mild
  3 (1.00%) high severe
sample-size/3d nested vector iteration 500
                        time:   [14.210 ms 14.237 ms 14.267 ms]
                        change: [-0.7055% -0.4232% -0.1433%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 300 measurements (3.33%)
  6 (2.00%) high mild
  4 (1.33%) high severe
sample-size/3d jagged array iteration 500
                        time:   [7.3264 ms 7.3520 ms 7.3807 ms]
                        change: [-0.1417% +0.3313% +0.8417%] (p = 0.18 > 0.05)
                        No change in performance detected.
Found 11 outliers among 300 measurements (3.67%)
  4 (1.33%) high mild
  7 (2.33%) high severe
sample-size/3d jagged array iteration 500(U32)
                        time:   [7.2936 ms 7.3134 ms 7.3355 ms]
                        change: [-0.9109% -0.5831% -0.2039%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 300 measurements (1.67%)
  2 (0.67%) high mild
  3 (1.00%) high severe
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue on GitHub.

---
