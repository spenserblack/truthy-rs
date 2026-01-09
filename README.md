# truthy
[![Crates.io](https://img.shields.io/crates/v/truthy)](https://crates.io/crates/truthy)
[![Docs.rs](https://docs.rs/truthy/badge.svg)](https://docs.rs/truthy)
![Crates.io](https://img.shields.io/crates/d/truthy)

Check if a value is "truthy"

## Behavior
```rust
// non-zero numbers are truthy
0u32.truthy() // false
0f32.truthy() // false
1u32.truthy() // true
1f32.truthy() // true

// empty strings are not truthy
"".truthy() // false
" ".truthy() // true

// Options are truthy if not None and their value is truthy
let none: Option<()> = None;
let falsy_inner = Some(false);
let truthy_inner = Some(true);
none.truthy() // false
falsy_inner.truthy() // false
truthy_inner.truthy() // true

// Results are truthy if Ok and value is truthy
let falsy_err: Result<(), _> = Err(false);
let truthy_err: Result<(), _> = Err(true);
let falsy_ok: Result<_, ()> = Ok(false);
let truthy_ok: Result<_, ()> = Ok(true);

falsy_err.truthy() // false
truthy_err.truthy() // false
falsy_ok.truthy() // false
truthy_ok.truthy() // true

// Empty vecs and arrays are falsy
let empty_array: [();0] = [];
let empty_vec: Vec<()> = Vec::new();

empty_array.truthy() // false
empty_vec.truthy() // false

// The truthy behavior of arrays and vecs also applies to tuples from size 0 to 12
let empty_tuple = ();
let not_empty_tuple = (1, "2", '3');
empty_tuple.truthy() // false
not_empty_tuple.truthy() // true
```

## `truthy!` macro
```rust
let my_bool = x.truthy() && y.truthy() || !z.truthy();
```
The above code can be a bit annoying, having to repeat `.truthy()` multiple times. The `truthy!`
macro appends `.truthy()` to save you time.
```rust
let my_bool = truthy!(x && y || !z);
```
You can run the [example][truthy! example] with `cargo run --example truthy_macro`.

### Limitations
The `truthy!` macro cannot take non-boolean expressions. The only valid tokens in `truthy!` are
`(`, `)`, `!`, `&&`, `||`, and identities. For example, the following would fail to compile.
```rust
truthy!(Some(1).unwrap() && 0 + 1);
```
To get around this, you would need to assign the expressions before using `truthy!`.
```rust
let x = Some(1).unwrap();
let y = 0 + 1;
truthy!(x && y);
```

## Features
### `and-or`
This crate has an `and-or` feature, which will provide the functions `truthy_and` and `truthy_or` to
any type that implements `Truthy` and `Sized`.
For example, `true.truthy_and("It was truthy!")` returns `Some("It was truthy!")`.
You can run the [example][and-or example] with `cargo run --features and-or --example and_or`.

[truthy! example]: https://github.com/spenserblack/truthy-rs/blob/master/examples/truthy_macro.rs
[and-or example]: https://github.com/spenserblack/truthy-rs/blob/master/examples/and_or.rs
