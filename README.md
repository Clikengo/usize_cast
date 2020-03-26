# usize_cast

Very simply library that allow compile time checked cast from and to `usize`/`isize`.

## Installation

```toml
[dependencies]
usize_cast = "1.0.0"
```

## Usage

```rust
use usize_cast::{IntoUsize, FromUsize, IntoIsize, FromIsize};

assert_eq!(1u64.into_usize(), 1usize);
assert_eq!(2i64.into_isize(), 2isize);
assert_eq!(3i64.into_isize(), 3isize);
assert_eq!(u64::max_value().into_usize(), usize::max_value());
assert_eq!(i64::max_value().into_isize(), isize::max_value());
assert_eq!(i64::min_value().into_isize(), isize::min_value());

assert_eq!(1u64, u64::from_usize(1usize));
assert_eq!(2i64, i64::from_isize(2isize));
assert_eq!(3i64, i64::from_isize(3isize));
assert_eq!(u64::max_value(), u64::from_usize(usize::max_value()));
assert_eq!(i64::max_value(), i64::from_isize(isize::max_value()));
assert_eq!(i64::min_value(), i64::from_isize(isize::min_value()));
```
