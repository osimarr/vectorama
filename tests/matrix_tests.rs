#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::{matrix::*, vector::vec3::Vec3};

    #[test]
    fn test_matrix_zeros() {
        const M: usize = 3;
        const N: usize = 3;
        let zeros: Matrix<M, N> = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                assert_relative_eq!(0.0, zeros[(m, n)], epsilon = f32::EPSILON)
            }
        }
    }

    #[test]
    fn test_matrix_indexing() {
        // column major matrix
        let matrix_array = [
            [1.0, 2.0, 3.0], // Column 0
            [4.0, 5.0, 6.0], // Column 1
        ];
        let matrix = Matrix::from(matrix_array);
        for m in 0..3 {
            for n in 0..2 {
                assert_relative_eq!(matrix[(m, n)], matrix_array[n][m], epsilon = f32::EPSILON)
            }
        }
    }

    #[test]
    fn test_matrix_from_flattened() {
        // column major matrix
        let matrix_array: [f32; 6] = [
            1.0, 2.0, 3.0, // Column 0
            4.0, 5.0, 6.0, // Column 1
        ];
        let matrix: Matrix<3, 2> = Matrix::from_flattened(&matrix_array);
        for m in 0..3 {
            for n in 0..2 {
                assert_relative_eq!(
                    matrix[(m, n)],
                    matrix_array[n * 3 + m],
                    epsilon = f32::EPSILON
                );
            }
        }
    }

    #[test]
    fn test_matrix_view() {
        // column major matrix
        let matrix_array = [
            [1.0, 2.0, 3.0], // Column 0
            [4.0, 5.0, 6.0], // Column 1
        ];
        let matrix = Matrix::from(matrix_array);
        const VM: usize = 2;
        const VN: usize = 2;
        let start_m = 1;
        let start_n = 0;
        let view = matrix.view::<VM, VM>(start_m, start_n);
        for m in 0..VM {
            for n in 0..VN {
                assert_relative_eq!(
                    matrix[(m + start_m, n + start_n)],
                    view[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }
    }

    #[test]
    fn test_matrix_identity() {
        let identity: Matrix<4, 4> = Matrix::identity();
        for m in 0..4 {
            for n in 0..4 {
                if m == n {
                    assert_relative_eq!(identity[(m, n)], 1.0, epsilon = f32::EPSILON);
                } else {
                    assert_relative_eq!(identity[(m, n)], 0.0, epsilon = f32::EPSILON);
                }
            }
        }
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix_array = [
            [1.0, 2.0, 3.0], // Column 0
            [4.0, 5.0, 6.0], // Column 1
            [7.0, 8.0, 9.0], // Column 2
        ];
        let transpose_array = [
            [1.0, 4.0, 7.0], // Column 0
            [2.0, 5.0, 8.0], // Column 1
            [3.0, 6.0, 9.0], // Column 2
        ];
        let matrix = Matrix::from(matrix_array).transpose();
        let transpose = Matrix::from(transpose_array);
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(matrix[(m, n)], transpose[(m, n)], epsilon = f32::EPSILON);
            }
        }
    }

    #[test]
    fn test_matrix_multiplication() {
        #[rustfmt::skip]
        let a = [
            [ 1.0,  0.0,  7.0], // Column 0
            [-5.0, -2.0,  2.0], // Column 1
            [ 3.0,  6.0, -4.0], // Column 2
        ];
        #[rustfmt::skip]
        let b = [
            [-8.0,  7.0,  2.0], // Column 0
            [ 6.0,  0.0,  4.0], // Column 1
            [ 1.0, -3.0,  5.0], // Column 2
        ];
        #[rustfmt::skip]
        let ab = [
            [-37.0,  -2.0, -50.0], // Column 0
            [ 18.0,  24.0,  26.0], // Column 1
            [ 31.0,  36.0, -19.0], // Column 2
        ];

        let matrix_a = Matrix::from(a);
        let matrix_b = Matrix::from(b);
        let matrix_ab = Matrix::from(ab);
        let result = matrix_a * matrix_b;

        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(matrix_ab[(m, n)], result[(m, n)], epsilon = f32::EPSILON);
            }
        }
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        #[rustfmt::skip]
        let a = [
            [ 1.0,  0.0,  7.0], // Column 0
            [-5.0, -2.0,  2.0], // Column 1
            [ 3.0,  6.0, -4.0], // Column 2
        ];
        let b = 2.0f32;
        #[rustfmt::skip]
        let ba = [
            [  2.0,   0.0, 14.0], // Column 0
            [-10.0,  -4.0,  4.0], // Column 1
            [  6.0,  12.0, -8.0], // Column 2
        ];
        let matrix_a = Matrix::from(a);
        let matrix_ba = Matrix::from(ba);
        let result = b * matrix_a;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(matrix_ba[(m, n)], result[(m, n)], epsilon = f32::EPSILON);
            }
        }
        let mut result2 = matrix_a;
        result2 *= b;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(matrix_ba[(m, n)], result2[(m, n)], epsilon = f32::EPSILON);
            }
        }
    }

    #[test]
    fn test_matrix_scalar_division() {
        #[rustfmt::skip]
        let a = [
            [ 2.0,  4.0,  6.0], // Column 0
            [ 8.0, 10.0, 12.0], // Column 1
            [14.0, 16.0, 18.0], // Column 2
        ];
        let scalar = 2.0f32;
        #[rustfmt::skip]
        let expected = [
            [1.0, 2.0, 3.0], // Column 0
            [4.0, 5.0, 6.0], // Column 1
            [7.0, 8.0, 9.0], // Column 2
        ];
        let matrix_a = Matrix::from(a);
        let matrix_expected = Matrix::from(expected);

        let result = &matrix_a / scalar;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }

        let mut result2 = matrix_a;
        result2 /= scalar;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result2[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }
    }

    #[test]
    fn test_matrix_inverse() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 2.0, 3.0],
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let inverse_a = [
            [1.0, -2.0, 1.0, 0.0],
            [0.0, 1.0, -2.0, 1.0],
            [0.0, 0.0, 1.0, -2.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let matrix_a = Matrix::from(a);
        let inverse_matrix_a = Matrix::from(inverse_a);
        let inverse = matrix_a.try_inverse().unwrap();

        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(inverse_matrix_a[(m, n)], inverse[(m, n)]);
            }
        }
    }

    #[test]
    fn test_singular_matrix_inverse() {
        #[rustfmt::skip]
        let singular = [
            [1.0, 2.0, 3.0],
            [2.0, 4.0, 6.0],
            [3.0, 6.0, 9.0]
        ];

        let singular_matrix = Matrix::from(singular);
        assert!(singular_matrix.try_inverse().is_none())
    }

    #[test]
    fn test_matrix_determinant_2x2() {
        let a = [
            [1.0, 2.0], // Column 0
            [3.0, 4.0], // Column 1
        ];
        let matrix = Matrix::from(a);
        let det = matrix.determinant();
        assert_relative_eq!(det, -2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_matrix_determinant_3x3() {
        #[rustfmt::skip]
        let a = [
            [6.0,  1.0, 1.0], // Column 0
            [4.0, -2.0, 5.0], // Column 1
            [2.0,  8.0, 7.0], // Column 2
        ];
        let matrix = Matrix::from(a);
        let det = matrix.determinant();
        assert_relative_eq!(det, -306.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_matrix_determinant_4x4() {
        let a = [
            [1.0, 2.0, 3.0, 4.0], // Column 0
            [0.0, 1.0, 2.0, 3.0], // Column 1
            [0.0, 0.0, 1.0, 2.0], // Column 2
            [0.0, 0.0, 0.0, 1.0], // Column 3
        ];
        let matrix = Matrix::from(a);
        let det = matrix.determinant();
        assert_relative_eq!(det, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_matrix_addition() {
        let a = [
            [1.0, 2.0, 3.0], // Column 0
            [4.0, 5.0, 6.0], // Column 1
            [7.0, 8.0, 9.0], // Column 2
        ];
        let b = [
            [9.0, 8.0, 7.0], // Column 0
            [6.0, 5.0, 4.0], // Column 1
            [3.0, 2.0, 1.0], // Column 2
        ];
        let expected = [
            [10.0, 10.0, 10.0], // Column 0
            [10.0, 10.0, 10.0], // Column 1
            [10.0, 10.0, 10.0], // Column 2
        ];
        let matrix_a = Matrix::from(a);
        let matrix_b = Matrix::from(b);
        let matrix_expected = Matrix::from(expected);

        let result = matrix_a + matrix_b;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }

        let mut result2 = matrix_a;
        result2 += matrix_b;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result2[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }
    }

    #[test]
    fn test_matrix_subtraction() {
        let a = [
            [9.0, 8.0, 7.0], // Column 0
            [6.0, 5.0, 4.0], // Column 1
            [3.0, 2.0, 1.0], // Column 2
        ];
        let b = [
            [1.0, 2.0, 3.0], // Column 0
            [4.0, 5.0, 6.0], // Column 1
            [7.0, 8.0, 9.0], // Column 2
        ];
        #[rustfmt::skip]
        let expected = [
            [ 8.0,  6.0,  4.0], // Column 0
            [ 2.0,  0.0, -2.0], // Column 1
            [-4.0, -6.0, -8.0], // Column 2
        ];
        let matrix_a = Matrix::from(a);
        let matrix_b = Matrix::from(b);
        let matrix_expected = Matrix::from(expected);

        let result = matrix_a - matrix_b;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }

        let mut result2 = matrix_a;
        result2 -= matrix_b;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result2[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }
    }

    #[test]
    fn test_matrix_negation() {
        let a = [
            [1.0, -2.0, 3.0],  // Column 0
            [-4.0, 5.0, -6.0], // Column 1
            [7.0, -8.0, 9.0],  // Column 2
        ];
        let expected = [
            [-1.0, 2.0, -3.0], // Column 0
            [4.0, -5.0, 6.0],  // Column 1
            [-7.0, 8.0, -9.0], // Column 2
        ];
        let matrix_a = Matrix::from(a);
        let matrix_expected = Matrix::from(expected);

        let result = -&matrix_a;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }

        let result2 = -matrix_a;
        for m in 0..3 {
            for n in 0..3 {
                assert_relative_eq!(
                    matrix_expected[(m, n)],
                    result2[(m, n)],
                    epsilon = f32::EPSILON
                );
            }
        }
    }

    #[test]
    fn test_matrix_ones() {
        let ones: Matrix<2, 2> = Matrix::ones();
        for m in 0..2 {
            for n in 0..2 {
                assert_relative_eq!(ones[(m, n)], 1.0, epsilon = f32::EPSILON);
            }
        }
    }

    #[test]
    fn test_matrix_as_flattened() {
        let a = [
            [1.0, 2.0], // Column 0
            [3.0, 4.0], // Column 1
        ];
        let matrix = Matrix::from(a);
        let flattened = matrix.as_flattened();
        // The order depends on your as_flattened implementation, but let's check the values:
        assert!(flattened.contains(&1.0));
        assert!(flattened.contains(&2.0));
        assert!(flattened.contains(&3.0));
        assert!(flattened.contains(&4.0));
    }

    #[test]
    fn test_matrix_view_out_of_bounds_panics() {
        let matrix = Matrix::<3, 3>::ones();
        let result = std::panic::catch_unwind(|| {
            let _ = matrix.view::<2, 2>(2, 2);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_look_at() {
        let eye = Vec3::new(0.0, 0.0, 5.0);
        let target = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let view_matrix = Matrix::look_at(eye, target, up);

        #[rustfmt::skip]
        let expected = Matrix::from([
            [1.0, 0.0, 0.0, 0.0], // Column 0
            [0.0, 1.0, 0.0, 0.0], // Column 1
            [0.0, 0.0, 1.0, 0.0], // Column 2
            [0.0, 0.0, -5.0, 1.0], // Column 3
        ]);

        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(view_matrix[(m, n)], expected[(m, n)], epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_matrix_perspective() {
        let fov = 45.0_f32.to_radians();
        let aspect = 1920.0 / 1080.0;
        let near = 1.0;
        let far = 100.0;
        let persp = Matrix::<4, 4>::perspective(aspect, fov, near, far);

        let f = 1.0 / (fov / 2.0).tan();
        assert_relative_eq!(persp[(0, 0)], f / aspect, epsilon = 1e-6);
        assert_relative_eq!(persp[(1, 1)], f, epsilon = 1e-6);
        assert_relative_eq!(persp[(2, 2)], (far + near) / (near - far), epsilon = 1e-6);
        assert_relative_eq!(
            persp[(2, 3)],
            (2.0 * far * near) / (near - far),
            epsilon = 1e-6
        );
        assert_relative_eq!(persp[(3, 2)], -1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_matrix_orthographic() {
        let left = -1.0;
        let right = 1.0;
        let bottom = -1.0;
        let top = 1.0;
        let near = 1.0;
        let far = 10.0;
        let ortho = Matrix::<4, 4>::orthographic(left, right, bottom, top, near, far);

        assert_relative_eq!(ortho[(0, 0)], 1.0, epsilon = 1e-6);
        assert_relative_eq!(ortho[(1, 1)], 1.0, epsilon = 1e-6);
        assert_relative_eq!(ortho[(2, 2)], -0.22222222, epsilon = 1e-6);
        assert_relative_eq!(ortho[(3, 0)], -0.0, epsilon = 1e-6);
        assert_relative_eq!(ortho[(3, 1)], -0.0, epsilon = 1e-6);
        assert_relative_eq!(ortho[(3, 2)], -1.2222222, epsilon = 1e-6);
        assert_relative_eq!(ortho[(3, 3)], 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_matrix_determinant_identity() {
        let identity: Matrix<4, 4> = Matrix::identity();
        let det = identity.determinant();
        assert_relative_eq!(det, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_matrix_determinant_zero() {
        let zero: Matrix<3, 3> = Matrix::zeros();
        let det = zero.determinant();
        assert_relative_eq!(det, 0.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_matrix_inverse_identity() {
        let identity: Matrix<4, 4> = Matrix::identity();
        let inv = identity.try_inverse().unwrap();
        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(inv[(m, n)], identity[(m, n)], epsilon = f32::EPSILON);
            }
        }
    }

    #[test]
    fn test_matrix_inverse_product_is_identity() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 2.0, 3.0],
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let matrix_a = Matrix::from(a);
        let inv = matrix_a.try_inverse().unwrap();
        let product = matrix_a * inv;
        let identity = Matrix::<4, 4>::identity();
        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(product[(m, n)], identity[(m, n)], epsilon = 1e-5);
            }
        }
    }

    #[test]
    fn test_matrix_column_access() {
        // Create a 3x3 column-major matrix
        let data = [
            [1.0, 2.0, 3.0], // Column 0
            [4.0, 5.0, 6.0], // Column 1
            [7.0, 8.0, 9.0], // Column 2
        ];
        let matrix = Matrix::<3, 3>::from(data);

        // Test each column
        for col in 0..3 {
            let column = matrix.column(col);
            for row in 0..3 {
                assert_relative_eq!(column[row], data[col][row], epsilon = f32::EPSILON);
            }
        }
    }
}
