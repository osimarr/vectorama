#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::vector::vec2::Vec2;

    #[test]
    fn test_vec2_new_and_fields() {
        let v = Vec2::new(1.0, 2.0);
        assert_relative_eq!(v.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.y, 2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_from_array() {
        let v = Vec2::from([3.0, 4.0]);
        assert_relative_eq!(v.x, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.y, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let result = a + b;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 6.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_sub() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);
        let result = a - b;
        assert_relative_eq!(result.x, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_mul_scalar() {
        let v = Vec2::new(2.0, 3.0);
        let result = v * 4.0;
        assert_relative_eq!(result.x, 8.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 12.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_div_scalar() {
        let v = Vec2::new(8.0, 6.0);
        let result = v / 2.0;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_neg() {
        let v = Vec2::new(1.0, -2.0);
        let result = -v;
        assert_relative_eq!(result.x, -1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let dot = a.dot(&b);
        assert_relative_eq!(dot, 1.0 * 3.0 + 2.0 * 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_magnitude() {
        let v = Vec2::new(3.0, 4.0);
        let mag = v.magnitude();
        assert_relative_eq!(mag, 5.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(0.0, 5.0);
        let n = v.normalize();
        let mag = n.magnitude();
        assert_relative_eq!(mag, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.y, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_cross() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let cross = a.x * b.y - a.y * b.x;
        assert_relative_eq!(a.dot(&b), 11.0, epsilon = f32::EPSILON);
        assert_relative_eq!(cross, -2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec2_xyz() {
        let v = Vec2::new(1.0, 2.0);
        let vec3 = v.xyz(3.0);
        assert_relative_eq!(vec3.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(vec3.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(vec3.z, 3.0, epsilon = f32::EPSILON);
    }
}
