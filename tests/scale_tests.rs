#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::scale::{scale2::Scale2, scale3::Scale3};
    use vectorama::vector::vec2::Vec2;
    use vectorama::vector::vec3::Vec3;

    #[test]
    fn test_scale2_new_and_mul_vec2() {
        let s = Scale2::new(2.0, 3.0);
        let v = Vec2::new(4.0, 5.0);
        let result = s * v;
        assert_relative_eq!(result.x, 8.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 15.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_scale2_homogeneous_matrix_and_from_matrix() {
        let s = Scale2::new(2.0, 3.0);
        let mat = s.homogeneous_matrix();
        // For column-major, scale factors are on the diagonal
        assert_relative_eq!(mat[(0, 0)], 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(mat[(1, 1)], 3.0, epsilon = f32::EPSILON);

        let s2 = Scale2::from(mat);
        assert_relative_eq!(s2.x, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(s2.y, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_scale3_new_and_mul_vec3() {
        let s = Scale3::new(2.0, 3.0, 4.0);
        let v = Vec3::new(5.0, 6.0, 7.0);
        let result = s * v;
        assert_relative_eq!(result.x, 10.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 18.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 28.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_scale3_homogeneous_matrix_and_from_matrix() {
        let s = Scale3::new(2.0, 3.0, 4.0);
        let mat = s.homogeneous_matrix();
        // For column-major, scale factors are on the diagonal
        assert_relative_eq!(mat[(0, 0)], 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(mat[(1, 1)], 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(mat[(2, 2)], 4.0, epsilon = f32::EPSILON);

        let s2 = Scale3::from(mat);
        assert_relative_eq!(s2.x, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(s2.y, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(s2.z, 4.0, epsilon = f32::EPSILON);
    }
}
