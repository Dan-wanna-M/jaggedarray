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

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue on GitHub.

---
