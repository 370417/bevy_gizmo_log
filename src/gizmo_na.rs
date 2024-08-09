//! Functions for logging gizmos using nalgebra instead of glam.

use bevy_color::Color;
use nalgebra::{Matrix4, Unit, UnitQuaternion, Vector2, Vector3};

use crate::gizmo;

/// Gizmo log version of [`arc_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_2d).
pub fn arc_2d(
    position: Vector2<f32>,
    direction_angle: f32,
    arc_angle: f32,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    gizmo::arc_2d(position.into(), direction_angle, arc_angle, radius, color)
}

/// Gizmo log version of [`arc_3d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_3d).
pub fn arc_3d(
    angle: f32,
    radius: f32,
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::arc_3d(angle, radius, position.into(), rotation.into(), color)
}

/// Gizmo log version of [`arrow`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow).
pub fn arrow(start: Vector3<f32>, end: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::arrow(start.into(), end.into(), color)
}

/// Gizmo log version of [`arrow_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow_2d).
pub fn arrow_2d(start: Vector2<f32>, end: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::arrow_2d(start.into(), end.into(), color)
}

/// Gizmo log version of [`axes`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes).
pub fn axes(transform: Matrix4<f32>, base_length: f32) -> String {
    gizmo::axes(bevy_math::Mat4::from(transform), base_length)
}

/// Gizmo log version of [`axes_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes_2d).
pub fn axes_2d(transform: Matrix4<f32>, base_length: f32) -> String {
    gizmo::axes_2d(bevy_math::Mat4::from(transform), base_length)
}

/// Gizmo log version of [`circle`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle).
pub fn circle(
    position: Vector3<f32>,
    normal: Unit<Vector3<f32>>,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    gizmo::circle(
        position.into(),
        bevy_math::Dir3::new_unchecked(normal.into()),
        radius,
        color,
    )
}

/// Gizmo log version of [`circle_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle_2d).
pub fn circle_2d(position: Vector2<f32>, radius: f32, color: impl Into<Color>) -> String {
    gizmo::circle_2d(position.into(), radius, color)
}

/// Gizmo log version of [`cuboid`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.cuboid).
pub fn cuboid(transform: Matrix4<f32>, color: impl Into<Color>) -> String {
    gizmo::cuboid(bevy_math::Mat4::from(transform), color)
}

/// Gizmo log version of [`ellipse`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse).
pub fn ellipse(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    half_size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::ellipse(position.into(), rotation.into(), half_size.into(), color)
}

/// Gizmo log version of [`ellipse_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse_2d).
pub fn ellipse_2d(
    position: Vector2<f32>,
    angle: f32,
    half_size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::ellipse_2d(position.into(), angle, half_size.into(), color)
}

/// Gizmo log version of [`grid`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid).
pub fn grid(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    cell_count: Vector2<u32>,
    spacing: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::grid(
        position.into(),
        rotation.into(),
        cell_count.into(),
        spacing.into(),
        color,
    )
}

/// Gizmo log version of [`grid_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_2d).
pub fn grid_2d(
    position: Vector2<f32>,
    rotation: f32,
    cell_count: Vector2<u32>,
    spacing: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::grid_2d(
        position.into(),
        rotation,
        cell_count.into(),
        spacing.into(),
        color,
    )
}

/// Gizmo log version of [`grid_3d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_3d).
pub fn grid_3d(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    cell_count: Vector3<u32>,
    spacing: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::grid_3d(
        position.into(),
        rotation.into(),
        cell_count.into(),
        spacing.into(),
        color,
    )
}

/// Gizmo log version of [`line`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line).
pub fn line(start: Vector3<f32>, end: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::line(start.into(), end.into(), color)
}

/// Gizmo log version of [`line_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_2d).
pub fn line_2d(start: Vector2<f32>, end: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::line_2d(start.into(), end.into(), color)
}

/// Gizmo log version of [`line_gradient`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient).
pub fn line_gradient(
    start: Vector3<f32>,
    end: Vector3<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::line_gradient(start.into(), end.into(), start_color, end_color)
}

/// Gizmo log version of [`line_gradient_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient_2d).
pub fn line_gradient_2d(
    start: Vector2<f32>,
    end: Vector2<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::line_gradient_2d(start.into(), end.into(), start_color, end_color)
}

/// Gizmo log version of [`linestrip`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip).
pub fn linestrip(
    positions: impl IntoIterator<Item = Vector3<f32>>,
    color: impl Into<Color>,
) -> String {
    gizmo::linestrip(positions.into_iter().map(|v| v.into()), color)
}

/// Gizmo log version of [`linestrip_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_2d).
pub fn linestrip_2d(
    positions: impl IntoIterator<Item = Vector2<f32>>,
    color: impl Into<Color>,
) -> String {
    gizmo::linestrip_2d(positions.into_iter().map(|v| v.into()), color)
}

/// Gizmo log version of [`linestrip_gradient`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient).
pub fn linestrip_gradient<C: Into<Color>>(
    points: impl IntoIterator<Item = (Vector3<f32>, C)>,
) -> String {
    gizmo::linestrip_gradient(points.into_iter().map(|(v, c)| (v.into(), c)))
}

/// Gizmo log version of [`linestrip_gradient_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient_2d).
pub fn linestrip_gradient_2d<C: Into<Color>>(
    positions: impl IntoIterator<Item = (Vector2<f32>, C)>,
) -> String {
    gizmo::linestrip_gradient_2d(positions.into_iter().map(|(v, c)| (v.into(), c)))
}

/// Gizmo log version of [`long_arc_3d_between`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.long_arc_3d_between).
pub fn long_arc_3d_between(
    center: Vector3<f32>,
    from: Vector3<f32>,
    to: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::long_arc_3d_between(center.into(), from.into(), to.into(), color)
}

/// Gizmo log version of [`ray`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray).
pub fn ray(start: Vector3<f32>, vector: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::ray(start.into(), vector.into(), color)
}

/// Gizmo log version of [`ray_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_2d).
pub fn ray_2d(start: Vector2<f32>, vector: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::ray_2d(start.into(), vector.into(), color)
}

/// Gizmo log version of [`ray_gradient`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient).
pub fn ray_gradient(
    start: Vector3<f32>,
    vector: Vector3<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::ray_gradient(start.into(), vector.into(), start_color, end_color)
}

/// Gizmo log version of [`ray_gradient_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient_2d).
pub fn ray_gradient_2d(
    start: Vector2<f32>,
    vector: Vector2<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::ray_gradient_2d(start.into(), vector.into(), start_color, end_color)
}

/// Gizmo log version of [`rect`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect).
pub fn rect(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rect(position.into(), rotation.into(), size.into(), color)
}

/// Gizmo log version of [`rect_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect_2d).
pub fn rect_2d(
    position: Vector2<f32>,
    rotation: f32,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rect_2d(position.into(), rotation, size.into(), color)
}

/// Gizmo log version of [`rounded_cuboid`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_cuboid).
pub fn rounded_cuboid(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    size: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_cuboid(position.into(), rotation.into(), size.into(), color)
}

/// Gizmo log version of [`rounded_rect`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect).
pub fn rounded_rect(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_rect(position.into(), rotation.into(), size.into(), color)
}

/// Gizmo log version of [`rounded_rect_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect_2d).
pub fn rounded_rect_2d(
    position: Vector2<f32>,
    rotation: f32,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_rect_2d(position.into(), rotation, size.into(), color)
}

/// Gizmo log version of [`short_arc_3d_between`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.short_arc_3d_between).
pub fn short_arc_3d_between(
    center: Vector3<f32>,
    from: Vector3<f32>,
    to: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::short_arc_3d_between(center.into(), from.into(), to.into(), color)
}

/// Gizmo log version of [`sphere`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.sphere).
pub fn sphere(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    gizmo::sphere(position.into(), rotation.into(), radius, color)
}
