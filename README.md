# truthy

[![Crates.io](https://img.shields.io/crates/v/truthy)](https://crates.io/crates/truthy)
[![Docs.rs](https://docs.rs/truthy/badge.svg)](https://docs.rs/truthy)

Check if a value is "truthy"

## Behavior

```rust
// non-zero numbers are truthy
0u32.truthy(); // false
0f32.truthy(); // false
1u32.truthy(); // true
1f32.truthy(); // true

// empty strings are not truthy
"".truthy(); // false
"foo".truthy(); // true
```
