use std::ops::{Add, Index, IndexMut, Mul, Neg, Range, RangeFrom, RangeFull, RangeTo, Sub};

/// A vector type for field computations.
///
/// Supports element-wise operations, scalar arithmetic, and mathematical functions
/// commonly needed for PDE solving. I tried using Rayon for parallelization, but
/// it was slower than sequential operations.
#[derive(Debug, Clone)]
pub struct FieldVector {
    data: Vec<f64>,
}

impl FieldVector {
    /// Create a new FieldVector from raw data
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    /// Create a zero vector of given length
    pub fn zeros(length: usize) -> Self {
        Self {
            data: vec![0.0; length],
        }
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get an iterator over the elements
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }

    /// Apply power function element-wise
    pub fn powi(&self, power: i32) -> Self {
        Self {
            data: self.data.iter().map(|x| x.powi(power)).collect(),
        }
    }

    /// Apply exponential function element-wise
    pub fn exp(&self) -> Self {
        Self {
            data: self.data.iter().map(|x| x.exp()).collect(),
        }
    }
}

// =============================================================================
// Core Operations
// =============================================================================

impl Neg for FieldVector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            data: self.data.into_iter().map(|x| -x).collect(),
        }
    }
}

// Vector + Vector
impl Add for &FieldVector {
    type Output = FieldVector;

    fn add(self, other: &FieldVector) -> FieldVector {
        FieldVector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

// Vector - Vector
impl Sub for &FieldVector {
    type Output = FieldVector;

    fn sub(self, other: &FieldVector) -> FieldVector {
        FieldVector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

// Vector * Vector (element-wise)
impl Mul for &FieldVector {
    type Output = FieldVector;

    fn mul(self, other: &FieldVector) -> FieldVector {
        FieldVector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a * b)
                .collect(),
        }
    }
}

// Scalar * Vector
impl Mul<&FieldVector> for f64 {
    type Output = FieldVector;

    fn mul(self, other: &FieldVector) -> FieldVector {
        FieldVector {
            data: other.data.iter().map(|x| self * x).collect(),
        }
    }
}

// Vector * Scalar
impl Mul<f64> for &FieldVector {
    type Output = FieldVector;

    fn mul(self, scalar: f64) -> FieldVector {
        scalar * self
    }
}

// Vector - Scalar
impl Sub<f64> for &FieldVector {
    type Output = FieldVector;

    fn sub(self, scalar: f64) -> FieldVector {
        FieldVector {
            data: self.data.iter().map(|x| x - scalar).collect(),
        }
    }
}

// Scalar - Vector
impl Sub<&FieldVector> for f64 {
    type Output = FieldVector;

    fn sub(self, other: &FieldVector) -> FieldVector {
        FieldVector {
            data: other.data.iter().map(|x| self - x).collect(),
        }
    }
}

// =============================================================================
// Convenience operations (implemented via reference operations)
// =============================================================================

impl Add<FieldVector> for FieldVector {
    type Output = FieldVector;
    fn add(self, other: FieldVector) -> FieldVector {
        &self + &other
    }
}

impl Add<&FieldVector> for FieldVector {
    type Output = FieldVector;
    fn add(self, other: &FieldVector) -> FieldVector {
        &self + other
    }
}

impl Add<FieldVector> for &FieldVector {
    type Output = FieldVector;
    fn add(self, other: FieldVector) -> FieldVector {
        self + &other
    }
}

impl Sub<FieldVector> for FieldVector {
    type Output = FieldVector;
    fn sub(self, other: FieldVector) -> FieldVector {
        &self - &other
    }
}

impl Mul<FieldVector> for FieldVector {
    type Output = FieldVector;
    fn mul(self, other: FieldVector) -> FieldVector {
        &self * &other
    }
}

impl Mul<FieldVector> for &FieldVector {
    type Output = FieldVector;
    fn mul(self, other: FieldVector) -> FieldVector {
        self * &other
    }
}

impl Mul<FieldVector> for f64 {
    type Output = FieldVector;
    fn mul(self, other: FieldVector) -> FieldVector {
        self * &other
    }
}

impl Mul<f64> for FieldVector {
    type Output = FieldVector;
    fn mul(self, scalar: f64) -> FieldVector {
        &self * scalar
    }
}

// =============================================================================
// Indexing Operations
// =============================================================================

impl Index<usize> for FieldVector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for FieldVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<Range<usize>> for FieldVector {
    type Output = [f64];
    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl IndexMut<Range<usize>> for FieldVector {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.data[range]
    }
}

impl Index<RangeTo<usize>> for FieldVector {
    type Output = [f64];
    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl Index<RangeFrom<usize>> for FieldVector {
    type Output = [f64];
    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl Index<RangeFull> for FieldVector {
    type Output = [f64];
    fn index(&self, range: RangeFull) -> &Self::Output {
        &self.data[range]
    }
}
