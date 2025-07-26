#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::vector::vec3::Vec3;

    #[test]
    fn test_vec3_new_and_fields() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_relative_eq!(v.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.z, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_from_array() {
        let v = Vec3::from([4.0, 5.0, 6.0]);
        assert_relative_eq!(v.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.y, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.z, 6.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a + b;
        assert_relative_eq!(result.x, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 7.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 9.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_sub() {
        let a = Vec3::new(7.0, 8.0, 9.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let result = a - b;
        assert_relative_eq!(result.x, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 6.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_mul_scalar() {
        let v = Vec3::new(2.0, 3.0, 4.0);
        let result = v * 2.0;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 8.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_div_scalar() {
        let v = Vec3::new(8.0, 6.0, 4.0);
        let result = v / 2.0;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let result = -v;
        assert_relative_eq!(result.x, -1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, -3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_swizzle_xy() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let xy = v.xy();
        assert_relative_eq!(xy.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xy.y, 2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_swizzle_yz() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let yz = v.yz();
        assert_relative_eq!(yz.x, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(yz.y, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_swizzle_xz() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let xz = v.xz();
        assert_relative_eq!(xz.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xz.y, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 6.0);
        let dot = a.dot(&b);
        assert_relative_eq!(
            dot,
            1.0 * 4.0 + 2.0 * -5.0 + 3.0 * 6.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn test_vec3_magnitude() {
        let v = Vec3::new(3.0, 4.0, 12.0);
        let mag = v.magnitude();
        assert_relative_eq!(mag, 13.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_normalize() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        let n = v.normalize();
        let mag = n.magnitude();
        assert_relative_eq!(mag, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.y, 3.0 / 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.z, 4.0 / 5.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_cross() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let cross = a.cross(&b);
        assert_relative_eq!(cross.x, -3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(cross.y, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(cross.z, -3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec3_xyzw() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = 4.0;
        let vec4 = v.xyzw(w);
        assert_relative_eq!(vec4.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(vec4.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(vec4.z, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(vec4.w, 4.0, epsilon = f32::EPSILON);
    }
}
