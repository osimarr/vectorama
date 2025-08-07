use crate::{matrix::Matrix, vector::vec3::Vec3};

impl<const M: usize> Default for Matrix<M, M> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<const M: usize> Matrix<M, M> {
    /// Returns the identity matrix of size `M x M`.
    ///
    /// # Returns
    /// A matrix with ones on the diagonal and zeros elsewhere.
    pub fn identity() -> Self {
        let mut identity = Self::zeros();
        for m in 0..M {
            identity[(m, m)] = 1.0;
        }
        identity
    }

    /// Attempts to compute the inverse of the matrix using Gauss-Jordan elimination.
    ///
    /// # Returns
    /// `Some(inverse)` if the matrix is invertible, or `None` if it is singular.
    pub fn try_inverse(&self) -> Option<Self> {
        if M < 2 {
            // no inverse if matrix is too small
            return None;
        }
        // Create a row-major augmented matrix for Gauss-Jordan elimination
        let mut extended = vec![vec![0.0; M * 2]; M];
        for m in 0..M {
            for n in 0..M {
                // Correctly copy from column-major `self` to row-major `extended`
                extended[m][n] = self[(m, n)];
            }
            extended[m][M + m] = 1.0;
        }

        // d = diagonal index (pivot column)
        for d in 0..M {
            // Find pivot row
            let mut pivot_index: usize = d;
            for m in d + 1..M {
                if extended[m][d].abs() > extended[pivot_index][d].abs() {
                    pivot_index = m;
                }
            }
            extended.swap(pivot_index, d);
            let factor = extended[d][d];

            if factor.abs() < f32::EPSILON {
                // singular matrix
                return None;
            }

            // Normalize pivot row (make pivot element 1.0)
            for n in d..M * 2 {
                extended[d][n] /= factor;
            }

            // Eliminate other entries in pivot column
            for m in 0..M {
                if m == d {
                    continue;
                }
                let value = extended[m][d];
                if value.abs() < f32::EPSILON {
                    continue;
                }
                for n in d..M * 2 {
                    extended[m][n] -= extended[d][n] * value;
                }
            }
        }

        // Copy the inverted part (right side of augmented matrix) back
        let mut inverse = Self::zeros();
        for m in 0..M {
            for n in 0..M {
                // Copy from row-major `extended` to column-major `inverse`
                inverse[(m, n)] = extended[m][n + M];
            }
        }

        Some(inverse)
    }
}

impl Matrix<2, 2> {
    /// Computes the determinant of a 2x2 matrix.
    ///
    /// # Returns
    /// The determinant value.
    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

impl Matrix<3, 3> {
    /// Computes the determinant of a 3x3 matrix.
    ///
    /// # Returns
    /// The determinant value.
    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * (self[(1, 1)] * self[(2, 2)] - self[(1, 2)] * self[(2, 1)])
            - self[(0, 1)] * (self[(1, 0)] * self[(2, 2)] - self[(1, 2)] * self[(2, 0)])
            + self[(0, 2)] * (self[(1, 0)] * self[(2, 1)] - self[(1, 1)] * self[(2, 0)])
    }
}

impl Matrix<4, 4> {
    /// Computes the determinant of a 4x4 matrix.
    ///
    /// # Returns
    /// The determinant value.
    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for col in 0..4 {
            // Build the minor for row 0, column `col`
            let mut minor = Matrix::<3, 3>::zeros();
            for m in 1..4 {
                let mut minor_col = 0;
                for n in 0..4 {
                    if n == col {
                        continue;
                    }
                    minor[(m - 1, minor_col)] = self[(m, n)];
                    minor_col += 1;
                }
            }
            // Calculate the cofactor sign
            let sign = if col % 2 == 0 { 1.0 } else { -1.0 };
            // Calculate the minor determinant using the Matrix<3,3> formula
            let minor_det = minor.determinant();
            det += sign * self[(0, col)] * minor_det;
        }
        det
    }

    /// Constructs a right-handed look-at view matrix.
    ///
    /// # Parameters
    /// - `eye`: The position of the camera.
    /// - `target`: The point the camera is looking at.
    /// - `up`: The up direction.
    ///
    /// # Returns
    /// A 4x4 view matrix in column-major order.
    pub fn look_at(eye: impl Into<Vec3>, target: impl Into<Vec3>, up: impl Into<Vec3>) -> Self {
        let eye: Vec3 = eye.into();
        let target: Vec3 = target.into();
        let up: Vec3 = up.into();

        // nalgebra's look_at_rh logic
        let f = (target - eye).normalize(); // forward
        let s = f.cross(&up).normalize(); // right
        let u = s.cross(&f); // up

        let mut view = Self::identity();

        // Assign basis vectors to columns for column-major matrix
        view[(0, 0)] = s.x;
        view[(0, 1)] = s.y;
        view[(0, 2)] = s.z;

        view[(1, 0)] = u.x;
        view[(1, 1)] = u.y;
        view[(1, 2)] = u.z;

        view[(2, 0)] = -f.x;
        view[(2, 1)] = -f.y;
        view[(2, 2)] = -f.z;

        view[(0, 3)] = -s.dot(&eye);
        view[(1, 3)] = -u.dot(&eye);
        view[(2, 3)] = f.dot(&eye);

        view
    }

    /// Constructs a perspective projection matrix.
    ///
    /// # Parameters
    /// - `aspect`: The aspect ratio (width / height).
    /// - `fov`: The vertical field of view in radians.
    /// - `near`: The near clipping plane.
    /// - `far`: The far clipping plane.
    ///
    /// # Returns
    /// A 4x4 perspective projection matrix in column-major order.
    pub fn perspective(aspect: f32, fov: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let mut perspective = Self::zeros();

        // Populate as a column-major matrix to match OpenGL's expectation
        perspective[(0, 0)] = f / aspect;
        perspective[(1, 1)] = f;
        perspective[(2, 2)] = (far + near) / (near - far);
        perspective[(3, 2)] = -1.0;
        perspective[(2, 3)] = (2.0 * far * near) / (near - far);
        // Note: perspective[(3, 3)] remains 0.0 from Self::zeros()

        perspective
    }

    /// Constructs an orthographic projection matrix.
    ///
    /// # Parameters
    /// - `left`, `right`: The left and right clipping planes.
    /// - `bottom`, `top`: The bottom and top clipping planes.
    /// - `near`, `far`: The near and far clipping planes.
    ///
    /// # Returns
    /// A 4x4 orthographic projection matrix in column-major order.
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut ortho = Self::zeros();
        ortho[(0, 0)] = 2.0 / (right - left);
        ortho[(1, 1)] = 2.0 / (top - bottom);
        ortho[(2, 2)] = -2.0 / (far - near);
        ortho[(3, 0)] = -(right + left) / (right - left);
        ortho[(3, 1)] = -(top + bottom) / (top - bottom);
        ortho[(3, 2)] = -(far + near) / (far - near);
        ortho[(3, 3)] = 1.0;
        ortho
    }
}
