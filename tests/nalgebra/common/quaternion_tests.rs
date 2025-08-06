#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vectorama::quaternion::unit::UnitQuaternion;
    use vectorama::vector::vec3::Vec3;

    use super::super::na;

    #[test]
    fn test_unit_quaternion_from_axis_angle_with_nalgebra() {
        use na::{UnitQuaternion as NaUnitQuaternion, Vector3 as NaVec3};

        let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
        let na_axis = na::UnitVector3::new_normalize(NaVec3::new(axis.x, axis.y, axis.z));
        let angle = std::f32::consts::FRAC_PI_3;

        let uq = UnitQuaternion::from_axis_angle(axis, angle);
        let na_uq = NaUnitQuaternion::from_axis_angle(&na_axis, angle);

        assert_relative_eq!(uq.vector.x, na_uq.coords.x, epsilon = 1e-6);
        assert_relative_eq!(uq.vector.y, na_uq.coords.y, epsilon = 1e-6);
        assert_relative_eq!(uq.vector.z, na_uq.coords.z, epsilon = 1e-6);
        assert_relative_eq!(uq.scalar, na_uq.coords.w, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_from_euler_angles_with_nalgebra() {
        use na::{UnitQuaternion as NaUnitQuaternion, Vector3 as NaVec3};

        let x = 0.1; // pitch
        let y = 0.2; // yaw
        let z = 0.3; // roll

        let uq = UnitQuaternion::from_euler_angles(x, y, z);

        // Recreate YXZ intrinsic rotation with nalgebra
        let na_qy = NaUnitQuaternion::from_axis_angle(&NaVec3::y_axis(), y);
        let na_qx = NaUnitQuaternion::from_axis_angle(&NaVec3::x_axis(), x);
        let na_qz = NaUnitQuaternion::from_axis_angle(&NaVec3::z_axis(), z);
        let na_uq = na_qy * na_qx * na_qz;

        assert_relative_eq!(uq.vector.x, na_uq.coords.x, epsilon = 1e-6);
        assert_relative_eq!(uq.vector.y, na_uq.coords.y, epsilon = 1e-6);
        assert_relative_eq!(uq.vector.z, na_uq.coords.z, epsilon = 1e-6);
        assert_relative_eq!(uq.scalar, na_uq.coords.w, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_slerp_with_nalgebra() {
        use na::UnitQuaternion as NaUnitQuaternion;

        let uq1 = UnitQuaternion::from_x_axis(std::f32::consts::FRAC_PI_4);
        let uq2 = UnitQuaternion::from_y_axis(std::f32::consts::FRAC_PI_4);

        let na_uq1 = NaUnitQuaternion::from(uq1);
        let na_uq2 = NaUnitQuaternion::from(uq2);

        let uq_slerp = uq1.slerp(&uq2, 0.5);
        let na_uq_slerp = na_uq1.slerp(&na_uq2, 0.5);

        assert_relative_eq!(uq_slerp.vector.x, na_uq_slerp.coords.x, epsilon = 1e-6);
        assert_relative_eq!(uq_slerp.vector.y, na_uq_slerp.coords.y, epsilon = 1e-6);
        assert_relative_eq!(uq_slerp.vector.z, na_uq_slerp.coords.z, epsilon = 1e-6);
        assert_relative_eq!(uq_slerp.scalar, na_uq_slerp.coords.w, epsilon = 1e-6);
    }

    #[test]
    fn test_unit_quaternion_conversion_to_nalgebra_and_back() {
        use na::UnitQuaternion as NaUnitQuaternion;

        let uq = UnitQuaternion::from_euler_angles(0.1, 0.2, 0.3);
        let na_uq: NaUnitQuaternion<f32> = uq.into();
        let uq2: UnitQuaternion = na_uq.into();

        assert_relative_eq!(uq.vector.x, uq2.vector.x, epsilon = 1e-6);
        assert_relative_eq!(uq.vector.y, uq2.vector.y, epsilon = 1e-6);
        assert_relative_eq!(uq.vector.z, uq2.vector.z, epsilon = 1e-6);
        assert_relative_eq!(uq.scalar, uq2.scalar, epsilon = 1e-6);
    }
}
