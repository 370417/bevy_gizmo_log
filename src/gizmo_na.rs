//! Functions for logging gizmos using nalgebra instead of glam.

use bevy_color::Color;
use bevy_math::{Isometry2d, Isometry3d, Quat, Vec2, Vec3};
use nalgebra::{Isometry2, Isometry3, Matrix4, Vector2, Vector3};

use crate::gizmo;

/// Gizmo log version of [`arc_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_2d).
pub fn arc_2d(
    isometry: Isometry2<f32>,
    arc_angle: f32,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    gizmo::arc_2d(convert_2d(isometry), arc_angle, radius, color)
}

/// Gizmo log version of [`arc_3d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_3d).
pub fn arc_3d(
    angle: f32,
    radius: f32,
    isometry: Isometry3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::arc_3d(angle, radius, convert_3d(isometry), color)
}

/// Gizmo log version of [`arrow`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow).
pub fn arrow(start: Vector3<f32>, end: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::arrow(start.into(), end.into(), color)
}

/// Gizmo log version of [`arrow_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow_2d).
pub fn arrow_2d(start: Vector2<f32>, end: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::arrow_2d(start.into(), end.into(), color)
}

/// Gizmo log version of [`axes`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes).
pub fn axes(transform: Matrix4<f32>, base_length: f32) -> String {
    gizmo::axes(bevy_math::Mat4::from(transform), base_length)
}

/// Gizmo log version of [`axes_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes_2d).
pub fn axes_2d(transform: Matrix4<f32>, base_length: f32) -> String {
    gizmo::axes_2d(bevy_math::Mat4::from(transform), base_length)
}

/// Gizmo log version of [`circle`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle).
pub fn circle(isometry: Isometry3<f32>, radius: f32, color: impl Into<Color>) -> String {
    gizmo::circle(convert_3d(isometry), radius, color)
}

/// Gizmo log version of [`circle_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle_2d).
pub fn circle_2d(position: Vector2<f32>, radius: f32, color: impl Into<Color>) -> String {
    gizmo::circle_2d(position.into(), radius, color)
}

/// Gizmo log version of [`cuboid`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.cuboid).
pub fn cuboid(transform: Matrix4<f32>, color: impl Into<Color>) -> String {
    gizmo::cuboid(bevy_math::Mat4::from(transform), color)
}

/// Gizmo log version of [`ellipse`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse).
pub fn ellipse(
    isometry: Isometry3<f32>,
    half_size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::ellipse(convert_3d(isometry), half_size.into(), color)
}

/// Gizmo log version of [`ellipse_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse_2d).
pub fn ellipse_2d(
    isometry: Isometry2<f32>,
    half_size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::ellipse_2d(convert_2d(isometry), half_size.into(), color)
}

/// Gizmo log version of [`grid`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid).
pub fn grid(
    isometry: Isometry3<f32>,
    cell_count: Vector2<u32>,
    spacing: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::grid(
        convert_3d(isometry),
        cell_count.into(),
        spacing.into(),
        color,
    )
}

/// Gizmo log version of [`grid_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_2d).
pub fn grid_2d(
    isometry: Isometry2<f32>,
    cell_count: Vector2<u32>,
    spacing: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::grid_2d(
        convert_2d(isometry),
        cell_count.into(),
        spacing.into(),
        color,
    )
}

/// Gizmo log version of [`grid_3d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_3d).
pub fn grid_3d(
    isometry: Isometry3<f32>,
    cell_count: Vector3<u32>,
    spacing: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::grid_3d(
        convert_3d(isometry),
        cell_count.into(),
        spacing.into(),
        color,
    )
}

/// Gizmo log version of [`line`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line).
pub fn line(start: Vector3<f32>, end: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::line(start.into(), end.into(), color)
}

/// Gizmo log version of [`line_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_2d).
pub fn line_2d(start: Vector2<f32>, end: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::line_2d(start.into(), end.into(), color)
}

/// Gizmo log version of [`line_gradient`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient).
pub fn line_gradient(
    start: Vector3<f32>,
    end: Vector3<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::line_gradient(start.into(), end.into(), start_color, end_color)
}

/// Gizmo log version of [`line_gradient_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient_2d).
pub fn line_gradient_2d(
    start: Vector2<f32>,
    end: Vector2<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::line_gradient_2d(start.into(), end.into(), start_color, end_color)
}

/// Gizmo log version of [`linestrip`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip).
pub fn linestrip(
    positions: impl IntoIterator<Item = Vector3<f32>>,
    color: impl Into<Color>,
) -> String {
    gizmo::linestrip(positions.into_iter().map(|v| v.into()), color)
}

/// Gizmo log version of [`linestrip_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_2d).
pub fn linestrip_2d(
    positions: impl IntoIterator<Item = Vector2<f32>>,
    color: impl Into<Color>,
) -> String {
    gizmo::linestrip_2d(positions.into_iter().map(|v| v.into()), color)
}

/// Gizmo log version of [`linestrip_gradient`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient).
pub fn linestrip_gradient<C: Into<Color>>(
    points: impl IntoIterator<Item = (Vector3<f32>, C)>,
) -> String {
    gizmo::linestrip_gradient(points.into_iter().map(|(v, c)| (v.into(), c)))
}

/// Gizmo log version of [`linestrip_gradient_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient_2d).
pub fn linestrip_gradient_2d<C: Into<Color>>(
    positions: impl IntoIterator<Item = (Vector2<f32>, C)>,
) -> String {
    gizmo::linestrip_gradient_2d(positions.into_iter().map(|(v, c)| (v.into(), c)))
}

/// Gizmo log version of [`long_arc_3d_between`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.long_arc_3d_between).
pub fn long_arc_3d_between(
    center: Vector3<f32>,
    from: Vector3<f32>,
    to: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::long_arc_3d_between(center.into(), from.into(), to.into(), color)
}

/// Gizmo log version of [`ray`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray).
pub fn ray(start: Vector3<f32>, vector: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::ray(start.into(), vector.into(), color)
}

/// Gizmo log version of [`ray_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_2d).
pub fn ray_2d(start: Vector2<f32>, vector: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::ray_2d(start.into(), vector.into(), color)
}

/// Gizmo log version of [`ray_gradient`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient).
pub fn ray_gradient(
    start: Vector3<f32>,
    vector: Vector3<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::ray_gradient(start.into(), vector.into(), start_color, end_color)
}

/// Gizmo log version of [`ray_gradient_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient_2d).
pub fn ray_gradient_2d(
    start: Vector2<f32>,
    vector: Vector2<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::ray_gradient_2d(start.into(), vector.into(), start_color, end_color)
}

/// Gizmo log version of [`rect`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect).
pub fn rect(isometry: Isometry3<f32>, size: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::rect(convert_3d(isometry), size.into(), color)
}

/// Gizmo log version of [`rect_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect_2d).
pub fn rect_2d(isometry: Isometry2<f32>, size: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::rect_2d(convert_2d(isometry), size.into(), color)
}

/// Gizmo log version of [`rounded_cuboid`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_cuboid).
pub fn rounded_cuboid(
    isometry: Isometry3<f32>,
    size: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_cuboid(convert_3d(isometry), size.into(), color)
}

/// Gizmo log version of [`rounded_rect`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect).
pub fn rounded_rect(
    isometry: Isometry3<f32>,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_rect(convert_3d(isometry), size.into(), color)
}

/// Gizmo log version of [`rounded_rect_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect_2d).
pub fn rounded_rect_2d(
    isometry: Isometry2<f32>,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_rect_2d(convert_2d(isometry), size.into(), color)
}

/// Gizmo log version of [`short_arc_3d_between`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.short_arc_3d_between).
pub fn short_arc_3d_between(
    center: Vector3<f32>,
    from: Vector3<f32>,
    to: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::short_arc_3d_between(center.into(), from.into(), to.into(), color)
}

/// Gizmo log version of [`sphere`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.sphere).
pub fn sphere(isometry: Isometry3<f32>, radius: f32, color: impl Into<Color>) -> String {
    gizmo::sphere(convert_3d(isometry), radius, color)
}

fn convert_2d(isometry: Isometry2<f32>) -> Isometry2d {
    let (translation, rotation_angle): (Vec2, f32) = isometry.into();
    Isometry2d::new(translation, rotation_angle.into())
}

fn convert_3d(isometry: Isometry3<f32>) -> Isometry3d {
    let (translation, rotation): (Vec3, Quat) = isometry.into();
    Isometry3d::new(translation, rotation)
}

#[cfg(test)]
mod tests {
    use core::f32;

    use approx::assert_relative_eq;
    use bevy_math::Rot2;
    use gizmo::GizmoCommand;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn test_convert_2d() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        for _ in 0..1000 {
            let color = Color::srgb(1., 1., 1.);
            let (na_isometry, glam_isometry) = rand_isometry_2d(&mut rng);

            let na_str = ellipse_2d(na_isometry, Vector2::new(1., 2.), color);
            let glam_str = crate::gizmo::ellipse_2d(glam_isometry, Vec2::new(1., 2.), color);

            let na_command: GizmoCommand = ron::de::from_str(&na_str).unwrap();
            let glam_command: GizmoCommand = ron::de::from_str(&glam_str).unwrap();

            match (na_command, glam_command) {
                (
                    GizmoCommand::Ellipse2d {
                        isometry: isometry1,
                        half_size: _,
                        color: _,
                    },
                    GizmoCommand::Ellipse2d {
                        isometry: isometry2,
                        half_size: _,
                        color: _,
                    },
                ) => {
                    assert_relative_eq!(isometry1.rotation.cos, isometry2.rotation.cos);
                    assert_relative_eq!(isometry1.rotation.sin, isometry2.rotation.sin);
                    assert_relative_eq!(isometry1.translation.x, isometry2.translation.x);
                    assert_relative_eq!(isometry1.translation.y, isometry2.translation.y);
                }
                _ => panic!("command should be ellipse2d"),
            }
        }
    }

    #[test]
    fn test_convert_3d() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        for _ in 0..1000 {
            let color = Color::srgb(1., 1., 1.);
            let (na_isometry, glam_isometry) = rand_isometry_3d(&mut rng);

            let na_str = ellipse(na_isometry, Vector2::new(1., 2.), color);
            let glam_str = crate::gizmo::ellipse(glam_isometry, Vec2::new(1., 2.), color);

            let na_command: GizmoCommand = ron::de::from_str(&na_str).unwrap();
            let glam_command: GizmoCommand = ron::de::from_str(&glam_str).unwrap();

            match (na_command, glam_command) {
                (
                    GizmoCommand::Ellipse {
                        isometry: isometry1,
                        half_size: _,
                        color: _,
                    },
                    GizmoCommand::Ellipse {
                        isometry: isometry2,
                        half_size: _,
                        color: _,
                    },
                ) => {
                    assert_relative_eq!(isometry1.rotation.x, isometry2.rotation.x);
                    assert_relative_eq!(isometry1.rotation.y, isometry2.rotation.y);
                    assert_relative_eq!(isometry1.rotation.z, isometry2.rotation.z);
                    assert_relative_eq!(isometry1.rotation.w, isometry2.rotation.w);
                    assert_relative_eq!(isometry1.translation.x, isometry2.translation.x);
                    assert_relative_eq!(isometry1.translation.y, isometry2.translation.y);
                    assert_relative_eq!(isometry1.translation.z, isometry2.translation.z);
                }
                _ => panic!("command should be ellipse"),
            }
        }
    }

    fn rand_isometry_2d(rng: &mut impl Rng) -> (Isometry2<f32>, Isometry2d) {
        let x = rng.gen();
        let y = rng.gen();
        let angle = f32::consts::TAU * rng.gen::<f32>();

        let na = Isometry2::new(Vector2::new(x, y), angle);
        let glam = Isometry2d::new(Vec2::new(x, y), Rot2::radians(angle));

        (na, glam)
    }

    fn rand_isometry_3d(rng: &mut impl Rng) -> (Isometry3<f32>, Isometry3d) {
        let x = rng.gen();
        let y = rng.gen();
        let z = rng.gen();

        let axis_angle_x = rng.gen();
        let axis_angle_y = rng.gen();
        let axis_angle_z = rng.gen();

        let axis_angle = Vector3::new(axis_angle_x, axis_angle_y, axis_angle_z);
        let scaled_axis = Vec3::new(axis_angle_x, axis_angle_y, axis_angle_z);

        let na = Isometry3::new(Vector3::new(x, y, z), axis_angle);
        let glam = Isometry3d::new(Vec3::new(x, y, z), Quat::from_scaled_axis(scaled_axis));

        (na, glam)
    }
}
