#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::translation::{translation2::Translation2, translation3::Translation3};

    use super::super::na;

    #[test]
    fn test_translation2_compare_with_nalgebra() {
        use na::geometry::Translation2 as NaTranslation2;

        let t = Translation2::new(1.0, 2.0);
        let na_t = NaTranslation2::new(1.0, 2.0);

        // Compare homogeneous matrices
        let mat = t.homogeneous_matrix();
        let na_mat = na_t.to_homogeneous();
        for i in 0..9 {
            assert_relative_eq!(
                mat.as_flattened()[i],
                na_mat.as_slice()[i],
                epsilon = f32::EPSILON
            );
        }

        // Compare translation extraction from matrix
        let t2 = Translation2::from(mat);
        assert_relative_eq!(t2.x, na_t.x, epsilon = f32::EPSILON);
        assert_relative_eq!(t2.y, na_t.y, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_translation3_compare_with_nalgebra() {
        use na::geometry::Translation3 as NaTranslation3;

        let t = Translation3::new(1.0, 2.0, 3.0);
        let na_t = NaTranslation3::new(1.0, 2.0, 3.0);

        // Compare homogeneous matrices
        let mat = t.homogeneous_matrix();
        let na_mat = na_t.to_homogeneous();
        for i in 0..16 {
            assert_relative_eq!(
                mat.as_flattened()[i],
                na_mat.as_slice()[i],
                epsilon = f32::EPSILON
            );
        }

        // Compare translation extraction from matrix
        let t2 = Translation3::from(mat);
        assert_relative_eq!(t2.x, na_t.x, epsilon = f32::EPSILON);
        assert_relative_eq!(t2.y, na_t.y, epsilon = f32::EPSILON);
        assert_relative_eq!(t2.z, na_t.z, epsilon = f32::EPSILON);
    }
}
