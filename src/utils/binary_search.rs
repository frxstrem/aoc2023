use std::cmp::Ordering;

/// Binary search for a range of values.
///
/// # Parameters
///
/// - `start`: The start of the range to search.
/// - `end`: The end of the range to search.
/// - `cmp(t)`: A function that compares `t` to the desired value.
///             It returns `Ordering::Less` if `t` is less than the desired
///             value, `Ordering::Greater` if `t` is greater than the desired
///             value, and `Ordering::Equal` if `t` is equal to the desired
///             value.
pub fn binary_search_range<T: Ord + Midpoint>(
    mut start: T,
    mut end: T,
    cmp: impl Fn(&T) -> Ordering,
) -> Result<T, T> {
    loop {
        if start == end {
            return Err(start);
        }

        let mid = start.midpoint(&end);

        if mid == start || mid == end {
            return Err(mid);
        }

        match cmp(&mid) {
            Ordering::Less => start = mid,
            Ordering::Greater => end = mid,
            Ordering::Equal => return Ok(mid),
        }
    }
}

/// A trait for types that can have a midpoint between any two values.
pub trait Midpoint {
    /// Returns the midpoint between `self` and `other`.
    ///
    /// If `self <= other` then `self <= self.midpoint(other) <= other`.
    /// If `self >= other` then `self >= self.midpoint(other) >= self`.
    ///
    /// If `self.midpoint(other)` is equal to `self` or `other`, then there
    /// are no other values between `self` and `other`.
    fn midpoint(&self, other: &Self) -> Self;
}

macro_rules! impl_mid_int {
    ($type:ty) => {
        impl Midpoint for $type {
            fn midpoint(&self, other: &Self) -> Self {
                if self < other {
                    self + (other - self) / 2
                } else {
                    other + (self - other) / 2
                }
            }
        }
    };
}

impl_mid_int!(u8);
impl_mid_int!(u16);
impl_mid_int!(u32);
impl_mid_int!(u64);
impl_mid_int!(u128);
impl_mid_int!(usize);

impl_mid_int!(i8);
impl_mid_int!(i16);
impl_mid_int!(i32);
impl_mid_int!(i64);
impl_mid_int!(i128);
impl_mid_int!(isize);
