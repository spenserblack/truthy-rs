//! Check if a value is "truthy"
//!
//! # Examples
//!
//! ```rust
//! use truthy::Truthy;
//!
//! assert!("".falsy());
//! assert!(1usize.truthy());
//!
//! let mut count = 0u8;
//! assert_eq!(0, count.and_then(|n| n - 1));
//! count.or_eq(2);
//! count.and_then_eq(|n| *n - 1);
//! assert_eq!(1, count);
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

    /// Does nothing if `self` is truthy, or sets `self` to `default`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// let mut a = "";
    /// let mut b = "foo";
    /// a.or_eq("default");
    /// b.or_eq("default");
    /// assert_eq!("default", a);
    /// assert_eq!("foo", b);
    /// ```
    fn or_eq(&mut self, default: Self)
    where
        Self: Sized,
    {
        if self.falsy() {
            *self = default;
        }
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

    /// Does nothing `self` is truthy, or calls `f` sets `self` to the result.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// let mut a = "";
    /// let mut b = "foo";
    /// a.or_else_eq(|| "default");
    /// b.or_else_eq(|| "default");
    /// assert_eq!("default", a);
    /// assert_eq!("foo", b);
    /// ```
    fn or_else_eq<F>(&mut self, f: F)
    where
        Self: Sized,
        F: FnOnce() -> Self,
    {
        if self.falsy() {
            *self = f();
        }
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

    /// Sets `self` to `replacement` if `self` is truthy, otherwise does nothing.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// let mut a = "";
    /// let mut b = "foo";
    /// a.and_eq("replacement");
    /// b.and_eq("replacement");
    /// assert_eq!("", a);
    /// assert_eq!("replacement", b);
    /// ```
    fn and_eq(&mut self, replacement: Self)
    where
        Self: Sized,
    {
        if self.truthy() {
            *self = replacement;
        }
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

    /// Calls `f` and sets `self` to the result if `self` is truthy, otherwise does nothing.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// let mut a = 0u8;
    /// let mut b = 2u8;
    /// a.and_then_eq(|n| *n - 1);
    /// b.and_then_eq(|n| *n - 1);
    /// assert_eq!(0, a);
    /// assert_eq!(1, b);
    /// ```
    fn and_then_eq<F>(&mut self, f: F)
    where
        Self: Sized,
        F: FnOnce(&Self) -> Self,
    {
        if self.truthy() {
            *self = f(self);
        }
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
    fn test_or_eq() {
        let mut a = "";
        let mut b = "original";
        a.or_eq("default");
        b.or_eq("default");
        assert_eq!("default", a);
        assert_eq!("original", b);
    }

    #[test]
    fn test_or_else() {
        assert_eq!("default", "".or_else(|| "default"));
        assert_eq!("original", "original".or_else(|| "default"));
    }

    #[test]
    fn test_or_else_eq() {
        let mut a = "";
        let mut b = "original";
        a.or_else_eq(|| "default");
        b.or_else_eq(|| "default");
        assert_eq!("default", a);
        assert_eq!("original", b);
    }

    #[test]
    fn test_and() {
        assert_eq!(0, 0u8.and(2));
        assert_eq!(2, 1u8.and(2));
    }

    #[test]
    fn test_and_eq() {
        let mut a = 0u8;
        let mut b = 1u8;
        a.and_eq(2);
        b.and_eq(2);
        assert_eq!(0, a);
        assert_eq!(2, b);
    }

    #[test]
    fn test_and_then() {
        assert_eq!(0, 0u8.and_then(|n| n - 1));
        assert_eq!(1, 2u8.and_then(|n| n - 1));
    }

    #[test]
    fn test_and_then_eq() {
        let mut a = 0u8;
        let mut b = 2u8;
        a.and_then_eq(|n| *n - 1);
        b.and_then_eq(|n| *n - 1);
        assert_eq!(0, a);
        assert_eq!(1, b);
    }
}
