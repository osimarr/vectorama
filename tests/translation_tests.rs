#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::translation::{translation2::Translation2, translation3::Translation3};
    use vectorama::vector::vec2::Vec2;
    use vectorama::vector::vec3::Vec3;

    #[test]
    fn test_translation2_new_and_translate() {
        let t = Translation2::new(1.0, 2.0);
        let v = Vec2::new(3.0, 4.0);
        let result = t.translate(&v);
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 6.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_translation2_homogeneous_matrix_and_from_matrix() {
        let t = Translation2::new(1.0, 2.0);
        let mat = t.homogeneous_matrix();
        // For column-major, translation part is in the last row of each column
        assert_relative_eq!(mat[(0, 2)], 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(mat[(1, 2)], 2.0, epsilon = f32::EPSILON);

        let t2 = Translation2::from(mat);
        assert_relative_eq!(t2.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(t2.y, 2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_translation2_mul_vec2() {
        let t = Translation2::new(1.0, 2.0);
        let v = Vec2::new(3.0, 4.0);
        let result = t * v;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 6.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_translation3_new_and_translate() {
        let t = Translation3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        let result = t.translate(&v);
        assert_relative_eq!(result.x, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 7.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 9.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_translation3_homogeneous_matrix_and_from_matrix() {
        let t = Translation3::new(1.0, 2.0, 3.0);
        let mat = t.homogeneous_matrix();
        // For column-major, translation part is in the last row of each column
        assert_relative_eq!(mat[(0, 3)], 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(mat[(1, 3)], 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(mat[(2, 3)], 3.0, epsilon = f32::EPSILON);

        let t2 = Translation3::from(mat);
        assert_relative_eq!(t2.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(t2.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(t2.z, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_translation3_mul_vec3() {
        let t = Translation3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        let result = t * v;
        assert_relative_eq!(result.x, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 7.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 9.0, epsilon = f32::EPSILON);
    }
}
