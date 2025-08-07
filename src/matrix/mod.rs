use std::ops::{Index, IndexMut};

pub mod add;
pub mod div;
pub mod mul;
pub mod neg;
pub mod square;
pub mod sub;

/// A column-major, fixed-size matrix of `f32` values.
///
/// # Type Parameters
/// - `M`: Number of rows
/// - `N`: Number of columns
///
/// # Features
/// - Stores data in column-major order, matching OpenGL and glTF conventions.
/// - Provides convenient indexing, construction, and view methods.
/// - Designed for efficient use in graphics and linear algebra applications.
///
/// # Example
/// ```
/// use vectorama::Matrix;
/// let mat = Matrix::<3, 3>::ones();
/// ```
// Column-major matrix
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    data: [[f32; M]; N],
}

// Index and IndexMut -----------------------------------------------------------------------------
impl<const M: usize, const N: usize> Index<(usize, usize)> for Matrix<M, N> {
    type Output = f32;

    fn index(&self, (m, n): (usize, usize)) -> &Self::Output {
        &self.data[n][m]
    }
}

impl<const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<M, N> {
    fn index_mut(&mut self, (m, n): (usize, usize)) -> &mut Self::Output {
        &mut self.data[n][m]
    }
}
// Index and IndexMut -----------------------------------------------------------------------------

// From -------------------------------------------------------------------------------------------
impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn from_flattened(data: &[f32]) -> Self {
        assert!(
            data.len() == M * N,
            "Invalid slice size ({}) for Matrix<{M}, {N}>",
            data.len()
        );
        let mut matrix = Self::zeros();
        for n in 0..N {
            for m in 0..M {
                matrix[(m, n)] = data[n * M + m];
            }
        }
        matrix
    }
}

impl<const M: usize, const N: usize> From<[[f32; M]; N]> for Matrix<M, N> {
    fn from(data: [[f32; M]; N]) -> Self {
        Self { data }
    }
}
// From -------------------------------------------------------------------------------------------

// AsRef ------------------------------------------------------------------------------------------
impl<const M: usize, const N: usize> AsRef<[f32]> for Matrix<M, N> {
    fn as_ref(&self) -> &[f32] {
        self.as_flattened()
    }
}

impl<const M: usize, const N: usize> AsRef<[[f32; M]; N]> for Matrix<M, N> {
    fn as_ref(&self) -> &[[f32; M]; N] {
        &self.data
    }
}
// AsRef ------------------------------------------------------------------------------------------

impl<const M: usize, const N: usize> Matrix<M, N> {
    /// Creates a matrix filled with zeros.
    ///
    /// # Returns
    /// A new matrix where every element is `0.0`.
    pub const fn zeros() -> Self {
        Self {
            data: [[0.0; M]; N],
        }
    }

    /// Creates a matrix filled with ones.
    ///
    /// # Returns
    /// A new matrix where every element is `1.0`.
    pub const fn ones() -> Self {
        Self {
            data: [[1.0; M]; N],
        }
    }

    /// Returns a flattened slice of the matrix data in column-major order.
    ///
    /// # Returns
    /// A reference to the underlying data as a contiguous slice.
    pub fn as_flattened(&self) -> &[f32] {
        self.data.as_flattened()
    }

    /// Returns a submatrix (view) of the current matrix.
    ///
    /// # Parameters
    /// - `start_m`: Starting row index.
    /// - `start_n`: Starting column index.
    ///
    /// # Returns
    /// A new matrix of size `VM` x `VN` containing the selected view.
    ///
    /// # Panics
    /// Panics if the view exceeds the dimensions of the original matrix.
    pub fn view<const VM: usize, const VN: usize>(
        &self,
        start_m: usize,
        start_n: usize,
    ) -> Matrix<VM, VN> {
        assert!(
            VM + start_m <= M && VN + start_n <= N,
            "Matrix View exceeds dimensions of Matrix<{M}, {N}"
        );
        let mut matrix: Matrix<VM, VN> = Matrix::zeros();
        for m in 0..VM {
            for n in 0..VN {
                matrix[(m, n)] = self[(m + start_m, n + start_n)];
            }
        }
        matrix
    }

    /// Returns the transpose of the matrix.
    ///
    /// # Returns
    /// A new matrix with rows and columns swapped.
    pub fn transpose(&self) -> Matrix<N, M> {
        let mut transpose = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                transpose[(m, n)] = self[(n, m)];
            }
        }
        transpose
    }

    /// Returns a copy of the specified column as an array.
    ///
    /// # Parameters
    /// - `index`: The column index to extract.
    ///
    /// # Returns
    /// An array containing the elements of the specified column.
    ///
    /// # Panics
    /// Panics if `index` is out of bounds.
    pub fn column(&self, index: usize) -> [f32; M] {
        assert!(index < N, "Index out of bounds for column access");
        self.data[index]
    }

    /// Returns a raw pointer to the matrix data.
    ///
    /// # Returns
    /// A pointer to the first element of the matrix data.
    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }
}
