// Required imports for arithmetic operations
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

// Define the Scalar trait with supertraits
// Scalar trait
pub trait Scalar:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Debug
{
    fn zero() -> Self;
    fn one() -> Self;
}
// Implement Scalar for u32
impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

// Implement Scalar for i32
impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

// Implement Scalar for i64
impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

// Implement Scalar for f32
impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

// Implement Scalar for f64
impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

// Matrix data structure that implements the Scalar trait and Debug trait and allows assert_eq! to be used
#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    // Create a 1x1 matrix
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::one()]])
    }

    // Create a matrix filled with zeros
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    // Create an identity matrix
    pub fn identity(n: usize) -> Matrix<T> {
        let mut data = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            data[i][i] = T::one();
        }
        Matrix(data)
    }

    //MUL - Matrix multiplication
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }

        Some(Matrix(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(row1, row2)| {
                    row1.iter()
                        .zip(row2.iter())
                        .map(|(&val1, &val2)| val1 + val2)
                        .collect()
                })
                .collect(),
        ))
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }

        Some(Matrix(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(row1, row2)| {
                    row1.iter()
                        .zip(row2.iter())
                        .map(|(&val1, &val2)| val1 - val2)
                        .collect()
                })
                .collect(),
        ))
    }
}

impl<T: Scalar> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let mut result = Matrix::zero(self.number_of_rows(), rhs.number_of_cols());

        for i in 0..self.number_of_rows() {
            for j in 0..rhs.number_of_cols() {
                let mut sum = T::zero();
                for k in 0..self.number_of_cols() {
                    sum = sum + self.0[i][k] * rhs.0[k][j];
                }
                result.0[i][j] = sum;
            }
        }

        Some(result)
    }
}
