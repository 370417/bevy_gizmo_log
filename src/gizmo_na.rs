//! Functions for logging gizmos using nalgebra instead of glam.

use bevy_color::Color;
use nalgebra::{Matrix4, Unit, UnitQuaternion, Vector2, Vector3};

use crate::gizmo;

pub fn arc_2d(
    position: Vector2<f32>,
    direction_angle: f32,
    arc_angle: f32,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    gizmo::arc_2d(position.into(), direction_angle, arc_angle, radius, color)
}

pub fn arc_3d(
    angle: f32,
    radius: f32,
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::arc_3d(angle, radius, position.into(), rotation.into(), color)
}

pub fn arrow(start: Vector3<f32>, end: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::arrow(start.into(), end.into(), color)
}

pub fn arrow_2d(start: Vector2<f32>, end: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::arrow_2d(start.into(), end.into(), color)
}

pub fn axes(transform: Matrix4<f32>, base_length: f32) -> String {
    gizmo::axes(bevy_math::Mat4::from(transform), base_length)
}

pub fn axes_2d(transform: Matrix4<f32>, base_length: f32) -> String {
    gizmo::axes_2d(bevy_math::Mat4::from(transform), base_length)
}

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

pub fn circle_2d(position: Vector2<f32>, radius: f32, color: impl Into<Color>) -> String {
    gizmo::circle_2d(position.into(), radius, color)
}

pub fn cuboid(transform: Matrix4<f32>, color: impl Into<Color>) -> String {
    gizmo::cuboid(bevy_math::Mat4::from(transform), color)
}

pub fn ellipse(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    half_size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::ellipse(position.into(), rotation.into(), half_size.into(), color)
}

pub fn ellipse_2d(
    position: Vector2<f32>,
    angle: f32,
    half_size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::ellipse_2d(position.into(), angle, half_size.into(), color)
}

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

pub fn line(start: Vector3<f32>, end: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::line(start.into(), end.into(), color)
}

pub fn line_2d(start: Vector2<f32>, end: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::line_2d(start.into(), end.into(), color)
}

pub fn line_gradient(
    start: Vector3<f32>,
    end: Vector3<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::line_gradient(start.into(), end.into(), start_color, end_color)
}

pub fn line_gradient_2d(
    start: Vector2<f32>,
    end: Vector2<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::line_gradient_2d(start.into(), end.into(), start_color, end_color)
}

pub fn linestrip(
    positions: impl IntoIterator<Item = Vector3<f32>>,
    color: impl Into<Color>,
) -> String {
    gizmo::linestrip(positions.into_iter().map(|v| v.into()), color)
}

pub fn linestrip_2d(
    positions: impl IntoIterator<Item = Vector2<f32>>,
    color: impl Into<Color>,
) -> String {
    gizmo::linestrip_2d(positions.into_iter().map(|v| v.into()), color)
}

pub fn linestrip_gradient<C: Into<Color>>(
    points: impl IntoIterator<Item = (Vector3<f32>, C)>,
) -> String {
    gizmo::linestrip_gradient(points.into_iter().map(|(v, c)| (v.into(), c)))
}

pub fn linestrip_gradient_2d<C: Into<Color>>(
    positions: impl IntoIterator<Item = (Vector2<f32>, C)>,
) -> String {
    gizmo::linestrip_gradient_2d(positions.into_iter().map(|(v, c)| (v.into(), c)))
}

pub fn long_arc_3d_between(
    center: Vector3<f32>,
    from: Vector3<f32>,
    to: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::long_arc_3d_between(center.into(), from.into(), to.into(), color)
}

pub fn ray(start: Vector3<f32>, vector: Vector3<f32>, color: impl Into<Color>) -> String {
    gizmo::ray(start.into(), vector.into(), color)
}

pub fn ray_2d(start: Vector2<f32>, vector: Vector2<f32>, color: impl Into<Color>) -> String {
    gizmo::ray_2d(start.into(), vector.into(), color)
}

pub fn ray_gradient(
    start: Vector3<f32>,
    vector: Vector3<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::ray_gradient(start.into(), vector.into(), start_color, end_color)
}

pub fn ray_gradient_2d(
    start: Vector2<f32>,
    vector: Vector2<f32>,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    gizmo::ray_gradient_2d(start.into(), vector.into(), start_color, end_color)
}

pub fn rect(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rect(position.into(), rotation.into(), size.into(), color)
}

pub fn rect_2d(
    position: Vector2<f32>,
    rotation: f32,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rect_2d(position.into(), rotation, size.into(), color)
}

pub fn rounded_cuboid(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    size: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_cuboid(position.into(), rotation.into(), size.into(), color)
}

pub fn rounded_rect(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_rect(position.into(), rotation.into(), size.into(), color)
}

pub fn rounded_rect_2d(
    position: Vector2<f32>,
    rotation: f32,
    size: Vector2<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::rounded_rect_2d(position.into(), rotation, size.into(), color)
}

pub fn short_arc_3d_between(
    center: Vector3<f32>,
    from: Vector3<f32>,
    to: Vector3<f32>,
    color: impl Into<Color>,
) -> String {
    gizmo::short_arc_3d_between(center.into(), from.into(), to.into(), color)
}

pub fn sphere(
    position: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    gizmo::sphere(position.into(), rotation.into(), radius, color)
}
