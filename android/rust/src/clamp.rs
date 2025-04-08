use std::ops::{Add, Div, Mul, Sub};


pub fn clamp<T>(value: T, low: T, high: T) -> T
    where T: PartialOrd<T>
{
    if value < low { return low; }
    if value > high { return high; }
    return value;
}

pub fn map_range<T>(val: T, i1: T, i2: T, o1: T, o2: T) -> T
    where T: Copy +
    PartialOrd<T> +
    Add<Output=T> +
    Sub<Output=T> +
    Mul<Output=T> +
    Div<Output=T>
{
    return (val - i1) * (o2 - o1) / (i2 - i1) + o1;
}

pub fn map_range_clamp<T>(val: T, i1: T, i2: T, o1: T, o2: T) -> T
    where T: Copy +
    PartialOrd<T> +
    Add<Output=T> +
    Sub<Output=T> +
    Mul<Output=T> +
    Div<Output=T>
{
    return clamp(map_range(val, i1, i2, o1, o2), o1, o2);
}