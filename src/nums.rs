use std::borrow::Borrow;

use num_traits::{
    Float, 
    Num,
};


pub fn avg<T, V>(
    slice_: &[V],
) -> T
where 
    T: Float,
    T: std::ops::AddAssign<T>,
    V: Borrow<T>,
{
    let mut count = 0;
    let mut sum = T::zero();

    for (i, el) in slice_.iter().enumerate() {
        count = i;
        sum += *el.borrow();
    }
    sum / T::from(count + 1).unwrap()
}

pub fn avg_with<T>(
    v: &T,
    slice_: &[T],
) -> T 
where 
    T: Float,
    T: std::ops::AddAssign<T>,
{
    let mut count = 0;
    let mut sum = T::zero();

    for (i, el) in slice_.iter().enumerate() {
        count = i;
        sum += *el;
    }
    (sum + *v.borrow()) / T::from(count + 2).unwrap()
}

pub fn nz<T, V>(
    num: V,
    exc_value: V,
) -> V
where 
    T: Float,
    V: Borrow<T>,
{
    if num.borrow().is_nan() { exc_value } else { num }
}

pub fn coll_nz<C, T, V>(
    slice: &[V],
    exc_value: V,
) -> C
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
    C: FromIterator<V>
{
    slice
        .iter()
        .map(|num| nz(*num, exc_value))
        .collect()
}

pub fn normalize<'a, T, V>(
    slice: &[V],
    to_normalize: V,
    min_new: &T,
    max_new: &T,
) -> T
where 
    T: 'a,
    T: Float,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    V: Borrow<T>,
    V: std::fmt::Debug + std::fmt::Display,
{
    let mut min_historic = T::infinity();
    let mut max_historic = -T::infinity();

    for num in slice {
        min_historic = if num.borrow() < &min_historic {*num.borrow()} else {min_historic};
        max_historic = if num.borrow() > &max_historic {*num.borrow()} else {max_historic};
    };
    (*to_normalize.borrow() - min_historic)
        / (max_historic - min_historic)
        * (*max_new - *min_new)
        + *min_new
}

pub fn dz<T>(
    num: T,
) -> T 
where 
    T: Float,
{
    if num == T::zero() { T::from(1e-10).unwrap() } else { num }
}

pub fn coll_drop_nan<T, V, C>(
    vec: &[V],
) -> C
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
    C: FromIterator<V>,
{
    vec
        .iter()
        .filter(|x| !(**x).borrow().is_nan())
        .copied()
        .collect()
}

pub fn abs<T, V>(
    num: V,
) -> T 
where
    T: Num,
    T: PartialOrd,
    T: std::ops::Neg<Output = T>,
    T: Copy,
    V: Borrow<T>,
{
    if num.borrow() < &T::zero() { -*num.borrow() } else { *num.borrow() }
}

pub fn round_f<T, V>(
    num: V,
    precision: &usize,
) -> T
where 
    T: Float,
    V: Borrow<T>,
    V: Copy,
{
    let mult = T::from(10.0.powi(*precision as i32)).unwrap();
    (*num.borrow() * mult).round() / mult
}

pub fn coll_comp<'a, C, T, V>(
    slice: &'a[V],
    func: fn(&T) -> bool,
) -> C
where 
    T: Float,
    T: 'a,
    V: Borrow<T>,
    C: FromIterator<&'a T>
{
    slice
        .iter()
        .filter(|v| func((*v).borrow()))
        .map(|v| v.borrow())
        .collect()
}

pub fn sign<T, V>(
    num: V,
) -> T
where 
    T: Float,
    V: Borrow<T>,
{
    let num = *num.borrow();

    if num > T::zero() {
        T::one()
    } else if num < T::zero() {
        -T::one()
    } else {
        T::zero()
    }
}