#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::{matrix::*, vector::vec3::Vec3};

    use super::super::na;

    #[test]
    fn test_matrix_look_at_with_nalgebra_look_at_rh() {
        let eye = Vec3::new(1.0, 2.0, 3.0);
        let target = Vec3::new(4.0, 5.0, 6.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let view_matrix = Matrix::look_at(eye, target, up);

        let nalgebra_view_matrix = na::Matrix4::<f32>::look_at_rh(
            &na::Point3::from(eye),
            &na::Point3::from(target),
            &na::Vector3::from(up),
        );

        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(
                    view_matrix[(m, n)],
                    nalgebra_view_matrix[(m, n)],
                    epsilon = 1e-6
                );
            }
        }
        unsafe {
            let na_ptr = nalgebra_view_matrix.as_ptr();
            let ptr = view_matrix.as_ptr();
            for m in 0..16 {
                assert_relative_eq!(*ptr.add(m), *na_ptr.add(m), epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_matrix_perspective_with_nalgebra() {
        let fov = 0.7853982;
        let aspect = 1.7777778;
        let near = 0.1;
        let far = 1000.0;
        let persp = Matrix::<4, 4>::perspective(aspect, fov, near, far);

        let nalgebra_persp = na::Matrix4::<f32>::new_perspective(aspect, fov, near, far);

        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(persp[(m, n)], nalgebra_persp[(m, n)], epsilon = 1e-6);
            }
        }
        unsafe {
            let na_ptr = nalgebra_persp.as_ptr();
            let ptr = persp.as_ptr();
            for m in 0..16 {
                assert_relative_eq!(*ptr.add(m), *na_ptr.add(m), epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_matrix_inverse_with_nalgebra() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 1.0, 2.1, 3.0],
            [3.0, 5.5, 1.1, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let matrix_a = Matrix::from(a);
        let nalgebra_matrix_a = na::Matrix4::<f32>::from_column_slice(&a.concat());
        let inv = matrix_a.try_inverse().unwrap();
        let nalgebra_inv = nalgebra_matrix_a.try_inverse().unwrap();

        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(inv[(m, n)], nalgebra_inv[(m, n)], epsilon = 1e-6);
            }
        }
        unsafe {
            let na_ptr = nalgebra_inv.as_ptr();
            let ptr = inv.as_ptr();
            for m in 0..16 {
                assert_relative_eq!(*ptr.add(m), *na_ptr.add(m), epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_matrix4_to_nalgebra_and_back() {
        use na::Matrix4;

        // Create a Matrix<4, 4> with known values (column-major)
        let data = [
            [1.0, 2.0, 3.0, 4.0],     // Column 0
            [5.0, 6.0, 7.0, 8.0],     // Column 1
            [9.0, 10.0, 11.0, 12.0],  // Column 2
            [13.0, 14.0, 15.0, 16.0], // Column 3
        ];
        let matrix = Matrix::<4, 4>::from(data);

        // Convert to nalgebra::Matrix4<f32>
        let na_matrix: Matrix4<f32> = matrix.into();

        // Check values match (column-major)
        for col in 0..4 {
            for row in 0..4 {
                assert_relative_eq!(
                    na_matrix[(row, col)],
                    data[col][row],
                    epsilon = f32::EPSILON
                );
            }
        }

        // Convert back to Matrix<4, 4>
        let matrix2 = Matrix::<4, 4>::from(na_matrix);

        // Check values match original
        for m in 0..4 {
            for n in 0..4 {
                assert_relative_eq!(matrix2[(m, n)], matrix[(m, n)], epsilon = f32::EPSILON);
            }
        }
    }

    #[test]
    fn test_nalgebra_matrix4_to_matrix_and_back() {
        use na::Matrix4;

        // Create a nalgebra::Matrix4<f32> with known values (column-major)
        let na_data: [f32; 16] = [
            1.0, 2.0, 3.0, 4.0, // Column 0
            5.0, 6.0, 7.0, 8.0, // Column 1
            9.0, 10.0, 11.0, 12.0, // Column 2
            13.0, 14.0, 15.0, 16.0, // Column 3
        ];
        let na_matrix = Matrix4::from_column_slice(&na_data);

        // Convert to Matrix<4, 4>
        let matrix: Matrix<4, 4> = Matrix::from(na_matrix);

        // Check values match (column-major)
        for col in 0..4 {
            for row in 0..4 {
                assert_relative_eq!(
                    matrix[(row, col)],
                    na_matrix[(row, col)],
                    epsilon = f32::EPSILON
                );
            }
        }

        // Convert back to nalgebra::Matrix4<f32>
        let na_matrix2: Matrix4<f32> = matrix.into();

        // Check values match original
        for col in 0..4 {
            for row in 0..4 {
                assert_relative_eq!(
                    na_matrix2[(row, col)],
                    na_matrix[(row, col)],
                    epsilon = f32::EPSILON
                );
            }
        }
    }
}
