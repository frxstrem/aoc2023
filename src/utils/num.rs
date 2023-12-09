use std::{
    mem,
    ops::{Div, Mul, Rem},
};

pub fn gcd<T: Int>(mut a: T, mut b: T) -> T {
    while b != T::ZERO {
        mem::swap(&mut a, &mut b);
        b = b % a;
    }

    a
}

pub fn lcm<T: Int>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}

pub trait Int:
    Copy + Eq + Ord + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self>
{
    const ZERO: Self;
}

macro_rules! impl_int {
    ($type:ty) => {
        impl Int for $type {
            const ZERO: Self = 0;
        }
    };
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u128);
impl_int!(usize);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);
