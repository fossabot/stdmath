#![feature(step_trait)]

pub trait Pow {
    type Output;
    fn pow(self, exp: u32) -> Self::Output;
}

pub trait Zero: Sized + std::ops::Add<Self, Output = Self> {
    fn zero() -> Self;
}
pub trait One: Sized + std::ops::Mul<Self, Output = Self> {
    fn one() -> Self;
}

macro_rules! bulk_impl_traits {
    (@ $type:ty, $zero:expr, $one:expr) => {
        impl Zero for $type {
            #[inline]
            fn zero() -> Self {
                $zero
            }
        }
        impl One for $type {
            #[inline]
            fn one() -> Self {
                $one
            }
        }
    };
    ($type:ty, $zero:expr, $one:expr) => {
        bulk_impl_traits!(@ $type, $zero, $one);
        impl Pow for $type {
            type Output = Self;
            #[inline]
            fn pow(self, exp: u32) -> Self {
                <$type>::pow(self, exp)
            }
        }
    };
    (($($type:ty),+) => ($zero:expr, $one:expr)) => {
        $(bulk_impl_traits!($type, $zero, $one);)+
    };
    (@ ($($type:ty),+) => ($zero:expr, $one:expr)) => {
        $(bulk_impl_traits!(@ $type, $zero, $one);)+
    };
  }

bulk_impl_traits!((i8, i16, i32, i64, isize) => (0, 1));
bulk_impl_traits!((u8, u16, u32, u64, usize) => (0, 1));
bulk_impl_traits!(@ (f32, f64) => (0.0, 1.0));
#[cfg(has_i128)]
bulk_impl_traits!((i128, u128) => (0, 1));

#[inline]
pub fn sigma<T, R>(start: T, end: T, func: impl Fn(T) -> R) -> R
where
    T: std::iter::Step,
    R: std::iter::Sum,
{
    (start..=end).map(func).sum()
}

#[inline]
pub fn product<T, R>(min: T, max: T, func: impl Fn(T) -> R) -> R
where
    T: std::iter::Step,
    R: std::iter::Product,
{
    (min..=max).map(func).product()
}

#[inline]
pub fn factorial<T, R>(val: T) -> R
where
    T: One + std::iter::Step,
    R: From<T> + std::iter::Product,
{
    product(
        One::one(),
        val,
        #[inline]
        |x| x.into(),
    )
}

#[inline]
pub fn factorial_count<T>(val: T) -> usize
where
    T: One + std::iter::Step + Into<f32>,
{
    1_usize + sigma(T::one(), val, |n| n.into().log10()).floor() as usize
}

pub enum Method {
    Repeat,
    NoRepeat,
}

#[inline]
pub fn combination<T, R>(n: T, r: T, method: Method) -> R
where
    T: One + std::iter::Step + Zero + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    R: One
        + From<T>
        + Zero
        + std::ops::Mul<Output = R>
        + std::ops::Div<Output = R>
        + std::iter::Product,
{
    match method {
        _ if r > n => R::zero(), // FIXME!
        Method::NoRepeat if n == r || r == T::zero() => R::one(),
        Method::NoRepeat => {
            let top = factorial::<T, R>(n);

            let fact_r_ = factorial::<T, R>(r);
            let fact_nr = factorial::<T, R>(n - r);

            top / (fact_r_ * fact_nr)
        }
        Method::Repeat if r == T::zero() => R::zero(),
        Method::Repeat => {
            let top = factorial::<T, R>(n + r - T::one());

            let fact_r_ = factorial::<T, R>(r);
            let fact_n1 = factorial::<T, R>(n - T::one());

            top / (fact_r_ * fact_n1)
        }
    }
}

#[inline]
pub fn permutation<T, R>(n: T, r: T, method: Method) -> R
where
    T: One
        + std::iter::Step
        + Into<u32>
        + Zero
        + Copy
        + std::ops::Sub<Output = T>
        + std::cmp::PartialEq,
    R: One + From<T> + Zero + Pow<Output = R> + std::ops::Div<Output = R> + std::iter::Product,
{
    if r == T::zero() {
        R::one()
    } else if n == T::zero() {
        R::zero()
    } else {
        match method {
            Method::NoRepeat => {
                let fact_n_ = factorial::<T, R>(n);
                let fact_nr = factorial::<T, R>(n - r);

                fact_n_ / fact_nr
            }
            Method::Repeat => R::from(n).pow(r.into()),
        }
    }
}

#[inline]
pub fn binomial<T, R>(a: T, b: T, n: T) -> R
where
    T: One
        + std::iter::Step
        + Into<u32>
        + Zero
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>,
    R: One
        + From<T>
        + Zero
        + Pow<Output = R>
        + std::ops::Mul<Output = R>
        + std::ops::Div<Output = R>
        + std::iter::Sum
        + std::iter::Product,
{
    sigma(T::zero(), n, |r| {
        let comb = combination::<T, R>(n, r, Method::NoRepeat);
        let a_nr = R::from(a).pow((n - r).into());
        let b_r_ = R::from(b).pow(r.into());
        comb * a_nr * b_r_
    })
}

pub fn pascals(n: u32) -> Vec<Vec<u32>> {
    let mut result = vec![vec![1], vec![1, 1]];
    for row in 2..n as usize {
        let mut vec = vec![1, 0];
        for col in 1..row {
            vec[col] = result[row - 1][col] + result[row - 1][col - 1];
            vec.push(1);
        }
        result.push(vec);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sigma() {
        assert_eq!(sigma(0, 0, |x| x), 0);
        assert_eq!(sigma(1, 1, |x| x), 1);
        assert_eq!(sigma(1, 10, |x| x), 55);
        assert_eq!(sigma(1, 10, |x| u32::pow(x, 2)), 385);
    }
    #[test]
    fn test_product() {
        assert_eq!(product(0, 0, |x| x), 0);
        assert_eq!(product(1, 1, |x| x), 1);
        assert_eq!(product(1, 10, |x| x), 3628800);
        assert_eq!(product(1, 10, |x| u64::pow(x, 2)), 13168189440000);
    }
    #[test]
    fn test_factorial() {
        assert_eq!(factorial::<u8, u8>(0), 1);
        assert_eq!(factorial::<u8, u8>(1), 1);
        assert_eq!(factorial::<u8, u8>(5), 120);
        assert_eq!(factorial::<u8, u16>(6), 720);
        assert_eq!(factorial::<u8, u32>(9), 362880);
        assert_eq!(factorial::<u8, u32>(10), 3628800);
    }
    #[test]
    fn test_factorial_count() {
        assert_eq!(factorial_count(0u8), 1);
        assert_eq!(factorial_count(1u8), 1);
        assert_eq!(factorial_count(6u8), 3);
        assert_eq!(factorial_count(9u8), 6);
        assert_eq!(factorial_count(10u8), 7);
        assert_eq!(factorial_count(34u8), 39);
    }
    #[test]
    fn test_combination() {
        assert_eq!(combination::<u8, u8>(0, 0, Method::NoRepeat), 1);
        assert_eq!(combination::<u8, u8>(5, 0, Method::NoRepeat), 1);
        assert_eq!(combination::<u8, u8>(0, 5, Method::NoRepeat), 0);
        assert_eq!(combination::<u8, u8>(5, 3, Method::NoRepeat), 10);

        assert_eq!(combination::<u8, u8>(0, 0, Method::Repeat), 0);
        assert_eq!(combination::<u8, u8>(5, 0, Method::Repeat), 0);
        assert_eq!(combination::<u8, u8>(0, 5, Method::Repeat), 0);
        assert_eq!(combination::<u8, u64>(5, 3, Method::Repeat), 35);
    }
    #[test]
    fn test_permutation() {
        assert_eq!(permutation::<u8, u8>(0, 0, Method::NoRepeat), 1);
        assert_eq!(permutation::<u8, u8>(5, 0, Method::NoRepeat), 1);
        assert_eq!(permutation::<u8, u8>(0, 5, Method::NoRepeat), 0);
        assert_eq!(permutation::<u8, u8>(5, 3, Method::NoRepeat), 60);

        assert_eq!(permutation::<u8, u8>(0, 0, Method::Repeat), 1);
        assert_eq!(permutation::<u8, u8>(5, 0, Method::Repeat), 1);
        assert_eq!(permutation::<u8, u8>(0, 5, Method::Repeat), 0);
        assert_eq!(permutation::<u8, u8>(5, 3, Method::Repeat), 125);
    }
    #[test]
    fn test_binomial() {
        assert_eq!(binomial::<u8, u32>(7, 10, 5), u32::pow(7 + 10, 5));
        assert_eq!(binomial::<u8, u32>(2, 5, 2), u32::pow(2 + 5, 2));
    }
    #[test]
    fn test_pascals() {
        assert_eq!(
            pascals(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ]
        );
        assert_eq!(
            pascals(10),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
                vec![1, 6, 15, 20, 15, 6, 1],
                vec![1, 7, 21, 35, 35, 21, 7, 1],
                vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
                vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
            ]
        );
    }
}
