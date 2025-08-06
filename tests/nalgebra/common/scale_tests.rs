#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::scale::{scale2::Scale2, scale3::Scale3};

    use super::super::na;

    #[test]
    fn test_scale2_compare_with_nalgebra() {
        use na::geometry::Scale2 as NaScale2;

        let s = Scale2::new(2.0, 3.0);
        let na_s = NaScale2::new(2.0, 3.0);

        // Compare homogeneous matrices
        let mat = s.homogeneous_matrix();
        let na_mat = na_s.to_homogeneous();
        for i in 0..9 {
            assert_relative_eq!(
                mat.as_flattened()[i],
                na_mat.as_slice()[i],
                epsilon = f32::EPSILON
            );
        }

        // Compare scale extraction from matrix
        let s2 = Scale2::from(mat);
        assert_relative_eq!(s2.x, na_s.x, epsilon = f32::EPSILON);
        assert_relative_eq!(s2.y, na_s.y, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_scale3_compare_with_nalgebra() {
        use na::geometry::Scale3 as NaScale3;

        let s = Scale3::new(2.0, 3.0, 4.0);
        let na_s = NaScale3::new(2.0, 3.0, 4.0);

        // Compare homogeneous matrices
        let mat = s.homogeneous_matrix();
        let na_mat = na_s.to_homogeneous();
        for i in 0..16 {
            assert_relative_eq!(
                mat.as_flattened()[i],
                na_mat.as_slice()[i],
                epsilon = f32::EPSILON
            );
        }

        // Compare scale extraction from matrix
        let s2 = Scale3::from(mat);
        assert_relative_eq!(s2.x, na_s.x, epsilon = f32::EPSILON);
        assert_relative_eq!(s2.y, na_s.y, epsilon = f32::EPSILON);
        assert_relative_eq!(s2.z, na_s.z, epsilon = f32::EPSILON);
    }
}
