//! Check if a value is "truthy"
//!
//! # Examples
//!
//! ```rust
//! use truthy::Truthy;
//!
//! assert!("".falsy());
//! assert!(1usize.truthy());
//! ```

/// Checks if a value is truthy. This usually means that the value is non-zero and
/// non-empty.
pub trait Truthy {
    /// The value is truthy.
    fn truthy(&self) -> bool;

    /// Not truthy
    #[inline]
    fn falsy(&self) -> bool {
        !self.truthy()
    }

    /// Returns `self` if `self` is truthy, or returns `default`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// assert_eq!("default", "".or("default"));
    /// assert_eq!("foo", "foo".or("default"));
    /// ```
    fn or(self, default: Self) -> Self
    where
        Self: Sized,
    {
        if self.truthy() { self } else { default }
    }

    /// Returns `self` if `self` is truthy, or calls `f` and returns the result.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// assert_eq!("default", "".or_else(|| "default"));
    /// assert_eq!("foo", "foo".or_else(|| "default"));
    /// ```
    fn or_else<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> Self,
    {
        if self.truthy() { self } else { f() }
    }

    /// Returns `self` if `self` is falsy, or returns `replacement`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// assert_eq!("", "".and("replacement"));
    /// assert_eq!("replacement", "foo".and("replacement"));
    /// ```
    fn and(self, replacement: Self) -> Self
    where
        Self: Sized,
    {
        if self.falsy() { self } else { replacement }
    }

    /// Returns `self` if `self` is falsy, or calls `f` and returns the result.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// assert_eq!(0, 0u8.and_then(|n| n - 1));
    /// assert_eq!(1, 2u8.and_then(|n| n - 1));
    /// ```
    fn and_then<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(Self) -> Self,
    {
        if self.falsy() { self } else { f(self) }
    }
}

macro_rules! impl_truthy_num {
    ($type:ident) => {
        impl $crate::Truthy for $type {
            /// `true` if not equal to `0`
            #[inline]
            fn truthy(&self) -> bool {
                const FALSY: $type = 0;
                *self != FALSY
            }
        }
    };
}

impl_truthy_num!(i8);
impl_truthy_num!(i16);
impl_truthy_num!(i32);
impl_truthy_num!(i64);
impl_truthy_num!(i128);
impl_truthy_num!(isize);
impl_truthy_num!(u8);
impl_truthy_num!(u16);
impl_truthy_num!(u32);
impl_truthy_num!(u64);
impl_truthy_num!(u128);
impl_truthy_num!(usize);

impl Truthy for f32 {
    /// "truthy" if not `0.0`
    ///
    /// ```
    /// # use truthy::Truthy;
    /// assert!(0.1f32.truthy());
    /// assert!(!0.0f32.truthy());
    /// ```
    fn truthy(&self) -> bool {
        !self.eq(&0f32)
    }
}

impl Truthy for f64 {
    /// "truthy" if not `0.0`
    ///
    /// ```
    /// # use truthy::Truthy;
    /// assert!(0.1f64.truthy());
    /// assert!(!0.0f64.truthy());
    /// ```
    fn truthy(&self) -> bool {
        !self.eq(&0f64)
    }
}

impl Truthy for str {
    /// `true` if not empty.
    ///
    /// This implementation allows the `Truthy` utilities to be available for types
    /// that implement `Deref<Target=str>`, such as `String`.
    ///
    /// ```
    /// # use truthy::Truthy;
    /// assert!(String::from("").falsy());
    /// assert!(String::from("foo").truthy());
    /// ```
    #[inline]
    fn truthy(&self) -> bool {
        !self.is_empty()
    }
}

impl Truthy for &str {
    /// `true` if not empty.
    ///
    /// ```
    /// # use truthy::Truthy;
    /// assert!("".falsy());
    /// assert!("foo".truthy());
    /// ```
    #[inline]
    fn truthy(&self) -> bool {
        !self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod strings {
        use super::*;

        #[test]
        fn truthy() {
            assert!("I have value!".truthy());
        }

        #[test]
        fn falsy() {
            assert!("".falsy());
        }
    }
    mod i8 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1i8).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0i8).falsy())
        }
    }

    mod i16 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1i16).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0i16).falsy())
        }
    }
    mod i32 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1i32).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0i32).falsy())
        }
    }
    mod i64 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1i64).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0i64).falsy())
        }
    }
    mod i128 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1i128).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0i128).falsy())
        }
    }
    mod isize {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1isize).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0isize).falsy())
        }
    }
    mod u8 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1u8).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0u8).falsy())
        }
    }
    mod u16 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1u16).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0u16).falsy())
        }
    }
    mod u32 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1u32).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0u32).falsy())
        }
    }
    mod u64 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1u64).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0u64).falsy())
        }
    }
    mod u128 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1u128).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0u128).falsy())
        }
    }

    mod usize {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1usize).truthy())
        }

        #[test]
        fn falsy() {
            assert!((0usize).falsy())
        }
    }

    mod f32 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1.0f32).truthy())
        }

        #[test]
        fn falsy() {
            assert!(!(0.0f32).truthy())
        }
    }
    mod f64 {
        use super::*;

        #[test]
        fn truthy() {
            assert!((1.0f64).truthy())
        }

        #[test]
        fn falsy() {
            assert!(!(0.0f64).truthy())
        }
    }

    #[test]
    fn test_or() {
        assert_eq!("default", "".or("default"));
        assert_eq!("original", "original".or("default"));
    }

    #[test]
    fn test_or_else() {
        assert_eq!("default", "".or_else(|| "default"));
        assert_eq!("original", "original".or_else(|| "default"));
    }

    #[test]
    fn test_and() {
        assert_eq!(0, 0u8.and(2));
        assert_eq!(2, 1u8.and(2));
    }

    #[test]
    fn test_and_then() {
        assert_eq!(0, 0u8.and_then(|n| n - 1));
        assert_eq!(1, 2u8.and_then(|n| n - 1));
    }
}
