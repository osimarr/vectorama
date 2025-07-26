#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::vector::vec4::Vec4;

    #[test]
    fn test_vec4_new_and_fields() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_relative_eq!(v.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.z, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.w, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_from_array() {
        let v = Vec4::from([5.0, 6.0, 7.0, 8.0]);
        assert_relative_eq!(v.x, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.y, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.z, 7.0, epsilon = f32::EPSILON);
        assert_relative_eq!(v.w, 8.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_add() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(4.0, 3.0, 2.0, 1.0);
        let result = a + b;
        assert_relative_eq!(result.x, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.w, 5.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_sub() {
        let a = Vec4::new(5.0, 7.0, 9.0, 11.0);
        let b = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let result = a - b;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.w, 7.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_mul_scalar() {
        let v = Vec4::new(2.0, 3.0, 4.0, 5.0);
        let result = v * 2.0;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 8.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.w, 10.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_div_scalar() {
        let v = Vec4::new(8.0, 6.0, 4.0, 2.0);
        let result = v / 2.0;
        assert_relative_eq!(result.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.w, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_neg() {
        let v = Vec4::new(1.0, -2.0, 3.0, -4.0);
        let result = -v;
        assert_relative_eq!(result.x, -1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.z, -3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(result.w, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_magnitude() {
        let v = Vec4::new(2.0, 3.0, 6.0, 1.0);
        let mag = v.magnitude();
        assert_relative_eq!(
            mag,
            (2.0f32 * 2.0 + 3.0 * 3.0 + 6.0 * 6.0 + 1.0 * 1.0).sqrt(),
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn test_vec4_normalize() {
        let v = Vec4::new(0.0, 3.0, 4.0, 0.0);
        let n = v.normalize();
        let mag = n.magnitude();
        assert_relative_eq!(mag, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.y, 3.0 / 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.z, 4.0 / 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.w, 0.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_xy() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let xy = v.xy();
        assert_relative_eq!(xy.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xy.y, 2.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_xz() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let xz = v.xz();
        assert_relative_eq!(xz.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xz.y, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_xw() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let xw = v.xw();
        assert_relative_eq!(xw.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xw.y, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_yz() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let yz = v.yz();
        assert_relative_eq!(yz.x, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(yz.y, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_yw() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let yw = v.yw();
        assert_relative_eq!(yw.x, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(yw.y, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_zw() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let zw = v.zw();
        assert_relative_eq!(zw.x, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(zw.y, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_xyz() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let xyz = v.xyz();
        assert_relative_eq!(xyz.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xyz.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xyz.z, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_xyw() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let xyw = v.xyw();
        assert_relative_eq!(xyw.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xyw.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xyw.z, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_xzw() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let xzw = v.xzw();
        assert_relative_eq!(xzw.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xzw.y, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(xzw.z, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_vec4_swizzle_yzw() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let yzw = v.yzw();
        assert_relative_eq!(yzw.x, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(yzw.y, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(yzw.z, 4.0, epsilon = f32::EPSILON);
    }
}
