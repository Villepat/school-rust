// Required imports for arithmetic operations
use std::ops::{Add, Div, Mul, Sub};

// Define the Scalar trait with supertraits
pub trait Scalar:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Copy
{
    // Associated type to represent the scalar
    type Item;

    // Function to return the "zero" of the type
    fn zero() -> Self::Item;

    // Function to return the "one" of the type
    fn one() -> Self::Item;
}

// Implement Scalar for u32
impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

// Implement Scalar for u64
impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

// Implement Scalar for i32
impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

// Implement Scalar for i64
impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}

// Implement Scalar for f32
impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}

// Implement Scalar for f64
impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}

