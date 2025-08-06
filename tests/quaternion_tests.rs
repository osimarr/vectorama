#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::quaternion::{Quaternion, unit::UnitQuaternion};
    use vectorama::vector::vec3::Vec3;

    #[test]
    fn test_quaternion_new_and_fields() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let q = Quaternion::new(v, 4.0);
        assert_relative_eq!(q.vector.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.z, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.scalar, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_quaternion_identity() {
        let q = Quaternion::identity();
        assert_relative_eq!(q.vector.x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.y, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.z, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.scalar, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_quaternion_conjugate() {
        let q = Quaternion::new(Vec3::new(1.0, -2.0, 3.0), 4.0);
        let conj = q.conjugate();
        assert_relative_eq!(conj.vector.x, -1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(conj.vector.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(conj.vector.z, -3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(conj.scalar, 4.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_quaternion_magnitude() {
        let q = Quaternion::new(Vec3::new(2.0, 3.0, 6.0), 1.0);
        let mag = q.magnitude();
        assert_relative_eq!(
            mag,
            (2.0f32 * 2.0 + 3.0 * 3.0 + 6.0 * 6.0 + 1.0 * 1.0).sqrt(),
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn test_quaternion_normalize() {
        let q = Quaternion::new(Vec3::new(0.0, 3.0, 4.0), 0.0);
        let n = q.normalize();
        let mag = n.magnitude();
        assert_relative_eq!(mag, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.vector.x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.vector.y, 3.0 / 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.vector.z, 4.0 / 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(n.scalar, 0.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_quaternion_dot() {
        let q1 = Quaternion::new(Vec3::new(1.0, 2.0, 3.0), 4.0);
        let q2 = Quaternion::new(Vec3::new(5.0, 6.0, 7.0), 8.0);
        let dot = q1.dot(&q2);
        assert_relative_eq!(
            dot,
            1.0 * 5.0 + 2.0 * 6.0 + 3.0 * 7.0 + 4.0 * 8.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn test_quaternion_from_axis_angle() {
        let axis = Vec3::new(0.0, 1.0, 0.0);
        let angle = std::f32::consts::FRAC_PI_2;
        let q = Quaternion::from_axis_angle(axis, angle);
        assert_relative_eq!(q.vector.x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.y, (angle / 2.0).sin(), epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.z, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.scalar, (angle / 2.0).cos(), epsilon = f32::EPSILON);
    }

    #[test]
    fn test_unit_quaternion_rotate_vector_identity() {
        let q = UnitQuaternion::identity();
        let v = Vec3::new(1.0, 2.0, 3.0);
        let rotated = q.rotate_vector(v);
        assert_relative_eq!(rotated.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(rotated.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(rotated.z, 3.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_unit_quaternion_new_normalized() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        let scalar = 0.0;
        let uq = UnitQuaternion::new_normalized(v, scalar);
        let mag = uq.magnitude();
        assert_relative_eq!(mag, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_unit_quaternion_from_quaternion() {
        let q = Quaternion::new(Vec3::new(0.0, 3.0, 4.0), 0.0);
        let uq = UnitQuaternion::from(q);
        let mag = uq.magnitude();
        assert_relative_eq!(mag, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_quaternion_from_euler_angles() {
        let x = std::f32::consts::FRAC_PI_2;
        let y = std::f32::consts::FRAC_PI_2;
        let z = std::f32::consts::FRAC_PI_2;
        let q = Quaternion::from_euler_angles_yxz(x, y, z);
        // Should be normalized
        assert_relative_eq!(q.magnitude(), 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_unit_quaternion_to_axis_angle() {
        let axis = Vec3::new(0.0, 1.0, 0.0);
        let angle = std::f32::consts::FRAC_PI_2;
        let q = UnitQuaternion::from_axis_angle(axis, angle);
        let (out_axis, out_angle) = q.to_axis_angle();
        assert_relative_eq!(out_axis.x, axis.x, epsilon = f32::EPSILON);
        assert_relative_eq!(out_axis.y, axis.y, epsilon = f32::EPSILON);
        assert_relative_eq!(out_axis.z, axis.z, epsilon = f32::EPSILON);
        assert_relative_eq!(out_angle, angle, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_unit_quaternion_to_euler_angles() {
        let x = std::f32::consts::FRAC_PI_4;
        let y = std::f32::consts::FRAC_PI_4;
        let z = std::f32::consts::FRAC_PI_4;
        let q = UnitQuaternion::from_euler_angles(x, y, z);
        let euler = q.to_euler_angles();
        // Should be close to input angles (allowing for floating-point error)
        assert_relative_eq!(euler.x, x, epsilon = 1e-6);
        assert_relative_eq!(euler.y, y, epsilon = 1e-6);
        assert_relative_eq!(euler.z, z, epsilon = 1e-6);
    }

    #[test]
    fn test_quaternion_from_vec3_and_from_array() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let q = Quaternion::from(v);
        assert_relative_eq!(q.vector.x, 1.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.y, 2.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.vector.z, 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q.scalar, 0.0, epsilon = f32::EPSILON);

        let arr = [4.0, 5.0, 6.0, 7.0];
        let q2 = Quaternion::from(arr);
        assert_relative_eq!(q2.vector.x, 4.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q2.vector.y, 5.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q2.vector.z, 6.0, epsilon = f32::EPSILON);
        assert_relative_eq!(q2.scalar, 7.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_quaternion_mul() {
        let q1 = Quaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), std::f32::consts::FRAC_PI_2);
        let q2 = Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2);
        let q3 = q1 * q2;
        // Should be normalized
        assert_relative_eq!(q3.magnitude(), 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_unit_quaternion_mul() {
        let uq1 =
            UnitQuaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), std::f32::consts::FRAC_PI_2);
        let uq2 =
            UnitQuaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2);
        let uq3 = uq1 * uq2;
        assert_relative_eq!(uq3.magnitude(), 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_quaternion_mul_correctness() {
        // Rotation of 90 degrees around Y-axis
        let q1 = Quaternion::from_y_axis(std::f32::consts::FRAC_PI_2);
        // Rotation of 90 degrees around X-axis
        let q2 = Quaternion::from_x_axis(std::f32::consts::FRAC_PI_2);

        // Expected result of q1 * q2
        let expected = Quaternion::new(Vec3::new(0.5, 0.5, -0.5), 0.5);

        let result = q1 * q2;

        assert_relative_eq!(result.vector.x, expected.vector.x, epsilon = 1e-6);
        assert_relative_eq!(result.vector.y, expected.vector.y, epsilon = 1e-6);
        assert_relative_eq!(result.vector.z, expected.vector.z, epsilon = 1e-6);
        assert_relative_eq!(result.scalar, expected.scalar, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_rotation_matrix() {
        // 120 degrees around arbitrary axis (normalized)
        let axis = Vec3::new(1.0, 1.0, 1.0).normalize();
        let angle = 2.0 * std::f32::consts::PI / 3.0;
        let uq = UnitQuaternion::from_axis_angle(axis, angle);
        let rot = uq.rotation_matrix();

        // The expected rotation matrix for 120 degrees around (1,1,1) normalized
        // This is a well-known rotation, and the matrix should permute axes:
        // [ 0  0  1 ]
        // [ 1  0  0 ]
        // [ 0  1  0 ]
        // But allow for floating point error.
        assert_relative_eq!(rot[(0, 0)], 0.0, epsilon = 1e-6);
        assert_relative_eq!(rot[(0, 1)], 0.0, epsilon = 1e-6);
        assert_relative_eq!(rot[(0, 2)], 1.0, epsilon = 1e-6);

        assert_relative_eq!(rot[(1, 0)], 1.0, epsilon = 1e-6);
        assert_relative_eq!(rot[(1, 1)], 0.0, epsilon = 1e-6);
        assert_relative_eq!(rot[(1, 2)], 0.0, epsilon = 1e-6);

        assert_relative_eq!(rot[(2, 0)], 0.0, epsilon = 1e-6);
        assert_relative_eq!(rot[(2, 1)], 1.0, epsilon = 1e-6);
        assert_relative_eq!(rot[(2, 2)], 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_homogeneous_matrix() {
        // 45 degrees around Y axis
        let uq = UnitQuaternion::from_y_axis(std::f32::consts::FRAC_PI_4);
        let mat = uq.homogeneous_matrix();

        // The expected rotation matrix for 45 degrees around Y axis
        let c = (std::f32::consts::FRAC_PI_4).cos();
        let s = (std::f32::consts::FRAC_PI_4).sin();

        assert_relative_eq!(mat[(0, 0)], c, epsilon = 1e-6);
        assert_relative_eq!(mat[(0, 2)], s, epsilon = 1e-6);
        assert_relative_eq!(mat[(2, 0)], -s, epsilon = 1e-6);
        assert_relative_eq!(mat[(2, 2)], c, epsilon = 1e-6);
        assert_relative_eq!(mat[(1, 1)], 1.0, epsilon = 1e-6);
        assert_relative_eq!(mat[(3, 3)], 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_identity() {
        let uq = UnitQuaternion::identity();
        assert_relative_eq!(uq.vector.x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(uq.vector.y, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(uq.vector.z, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(uq.scalar, 1.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_unit_quaternion_from_rotation_matrix() {
        // 90 degrees around Z axis
        let angle = std::f32::consts::FRAC_PI_2;
        let uq = UnitQuaternion::from_z_axis(angle);
        let rot = uq.rotation_matrix();

        // Reconstruct from rotation matrix
        let uq2 = UnitQuaternion::from_rotation_matrix(&rot);
        assert_relative_eq!(uq2.vector.x, uq.vector.x, epsilon = 1e-6);
        assert_relative_eq!(uq2.vector.y, uq.vector.y, epsilon = 1e-6);
        assert_relative_eq!(uq2.vector.z, uq.vector.z, epsilon = 1e-6);
        assert_relative_eq!(uq2.scalar, uq.scalar, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_rotate_vector() {
        // 180 degrees around Z axis should flip X and Y
        let q = UnitQuaternion::from_z_axis(std::f32::consts::PI);
        let v = Vec3::new(1.0, 2.0, 0.0);
        let rotated = q.rotate_vector(v);
        assert_relative_eq!(rotated.x, -1.0, epsilon = 1e-6);
        assert_relative_eq!(rotated.y, -2.0, epsilon = 1e-6);
        assert_relative_eq!(rotated.z, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_mul_normalization() {
        let uq1 = UnitQuaternion::from_x_axis(std::f32::consts::FRAC_PI_4);
        let uq2 = UnitQuaternion::from_y_axis(std::f32::consts::FRAC_PI_4);
        let uq3 = uq1 * uq2;
        assert_relative_eq!(uq3.magnitude(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_quaternion_dot_with_self() {
        let q = Quaternion::new(Vec3::new(1.0, 2.0, 3.0), 4.0);
        let dot = q.dot(&q);
        assert_relative_eq!(
            dot,
            1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0,
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn test_unit_quaternion_slerp() {
        use std::f32::consts::FRAC_PI_2;

        // Interpolate between identity and 90-degree rotation around Z
        let uq1 = UnitQuaternion::identity();
        let uq2 = UnitQuaternion::from_z_axis(FRAC_PI_2);

        // Halfway interpolation (should be 45-degree rotation around Z)
        let uq_mid = uq1.slerp(&uq2, 0.5);

        // The expected quaternion for 45-degree rotation around Z
        let expected = UnitQuaternion::from_z_axis(FRAC_PI_2 * 0.5);

        assert_relative_eq!(uq_mid.vector.x, expected.vector.x, epsilon = 1e-6);
        assert_relative_eq!(uq_mid.vector.y, expected.vector.y, epsilon = 1e-6);
        assert_relative_eq!(uq_mid.vector.z, expected.vector.z, epsilon = 1e-6);
        assert_relative_eq!(uq_mid.scalar, expected.scalar, epsilon = 1e-6);
    }
}
