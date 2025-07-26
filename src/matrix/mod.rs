use std::ops::{Index, IndexMut};

pub mod add;
pub mod div;
pub mod mul;
pub mod neg;
pub mod square;
pub mod sub;

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
    pub const fn zeros() -> Self {
        Self {
            data: [[0.0; M]; N],
        }
    }

    pub const fn ones() -> Self {
        Self {
            data: [[1.0; M]; N],
        }
    }

    pub fn as_flattened(&self) -> &[f32] {
        self.data.as_flattened()
    }

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

    pub fn transpose(&self) -> Matrix<N, M> {
        let mut transpose = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                transpose[(m, n)] = self[(n, m)];
            }
        }
        transpose
    }

    pub fn column(&self, index: usize) -> [f32; M] {
        assert!(index < N, "Index out of bounds for column access");
        self.data[index]
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }
}
