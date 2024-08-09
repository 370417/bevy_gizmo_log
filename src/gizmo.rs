//! Functions for logging gizmos.

use bevy_color::Color;
#[cfg(feature = "bevy")]
use bevy_gizmos::gizmos::Gizmos;
use bevy_math::{Dir3, Mat4, Quat, UVec2, UVec3, Vec2, Vec3};

use crate::transform::IntoMat4;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum GizmoCommand {
    Arc2d {
        position: Vec2,
        direction_angle: f32,
        arc_angle: f32,
        radius: f32,
        color: Color,
    },
    Arc3d {
        angle: f32,
        radius: f32,
        position: Vec3,
        rotation: Quat,
        color: Color,
    },
    Arrow {
        start: Vec3,
        end: Vec3,
        color: Color,
    },
    Arrow2d {
        start: Vec2,
        end: Vec2,
        color: Color,
    },
    Axes {
        transform: Mat4,
        base_length: f32,
    },
    Axes2d {
        transform: Mat4,
        base_length: f32,
    },
    Circle {
        position: Vec3,
        normal: Dir3,
        radius: f32,
        color: Color,
    },
    Circle2d {
        position: Vec2,
        radius: f32,
        color: Color,
    },
    Cuboid {
        transform: Mat4,
        color: Color,
    },
    Ellipse {
        position: Vec3,
        rotation: Quat,
        half_size: Vec2,
        color: Color,
    },
    Ellipse2d {
        position: Vec2,
        angle: f32,
        half_size: Vec2,
        color: Color,
    },
    Grid {
        position: Vec3,
        rotation: Quat,
        cell_count: UVec2,
        spacing: Vec2,
        color: Color,
    },
    Grid2d {
        position: Vec2,
        rotation: f32,
        cell_count: UVec2,
        spacing: Vec2,
        color: Color,
    },
    Grid3d {
        position: Vec3,
        rotation: Quat,
        cell_count: UVec3,
        spacing: Vec3,
        color: Color,
    },
    Line {
        start: Vec3,
        end: Vec3,
        color: Color,
    },
    Line2d {
        start: Vec2,
        end: Vec2,
        color: Color,
    },
    LineGradient {
        start: Vec3,
        end: Vec3,
        start_color: Color,
        end_color: Color,
    },
    LineGradient2d {
        start: Vec2,
        end: Vec2,
        start_color: Color,
        end_color: Color,
    },
    Linestrip {
        positions: Vec<Vec3>,
        color: Color,
    },
    Linestrip2d {
        positions: Vec<Vec2>,
        color: Color,
    },
    LinestripGradient {
        points: Vec<(Vec3, Color)>,
    },
    LinestripGradient2d {
        positions: Vec<(Vec2, Color)>,
    },
    LongArc3dBetween {
        center: Vec3,
        from: Vec3,
        to: Vec3,
        color: Color,
    },
    Ray {
        start: Vec3,
        vector: Vec3,
        color: Color,
    },
    Ray2d {
        start: Vec2,
        vector: Vec2,
        color: Color,
    },
    RayGradient {
        start: Vec3,
        vector: Vec3,
        start_color: Color,
        end_color: Color,
    },
    RayGradient2d {
        start: Vec2,
        vector: Vec2,
        start_color: Color,
        end_color: Color,
    },
    Rect {
        position: Vec3,
        rotation: Quat,
        size: Vec2,
        color: Color,
    },
    Rect2d {
        position: Vec2,
        rotation: f32,
        size: Vec2,
        color: Color,
    },
    RoundedCuboid {
        position: Vec3,
        rotation: Quat,
        size: Vec3,
        color: Color,
    },
    RoundedRect {
        position: Vec3,
        rotation: Quat,
        size: Vec2,
        color: Color,
    },
    RoundedRect2d {
        position: Vec2,
        rotation: f32,
        size: Vec2,
        color: Color,
    },
    ShortArc3dBetween {
        center: Vec3,
        from: Vec3,
        to: Vec3,
        color: Color,
    },
    Sphere {
        position: Vec3,
        rotation: Quat,
        radius: f32,
        color: Color,
    },
}

#[cfg(feature = "bevy")]
impl GizmoCommand {
    pub fn draw(self, gizmos: &mut Gizmos) {
        match self {
            Self::Arc2d {
                position,
                direction_angle,
                arc_angle,
                radius,
                color,
            } => {
                gizmos.arc_2d(position, direction_angle, arc_angle, radius, color);
            }
            Self::Arc3d {
                angle,
                radius,
                position,
                rotation,
                color,
            } => {
                gizmos.arc_3d(angle, radius, position, rotation, color);
            }
            Self::Arrow { start, end, color } => {
                gizmos.arrow(start, end, color);
            }
            Self::Arrow2d { start, end, color } => {
                gizmos.arrow_2d(start, end, color);
            }
            Self::Axes {
                transform,
                base_length,
            } => {
                gizmos.axes(transform, base_length);
            }
            Self::Axes2d {
                transform,
                base_length,
            } => {
                gizmos.axes_2d(transform, base_length);
            }
            Self::Circle {
                position,
                normal,
                radius,
                color,
            } => {
                gizmos.circle(position, normal, radius, color);
            }
            Self::Circle2d {
                position,
                radius,
                color,
            } => {
                gizmos.circle_2d(position, radius, color);
            }
            Self::Cuboid { transform, color } => {
                gizmos.cuboid(transform, color);
            }
            Self::Ellipse {
                position,
                rotation,
                half_size,
                color,
            } => {
                gizmos.ellipse(position, rotation, half_size, color);
            }
            Self::Ellipse2d {
                position,
                angle,
                half_size,
                color,
            } => {
                gizmos.ellipse_2d(position, angle, half_size, color);
            }
            Self::Grid {
                position,
                rotation,
                cell_count,
                spacing,
                color,
            } => {
                gizmos.grid(position, rotation, cell_count, spacing, color);
            }
            Self::Grid2d {
                position,
                rotation,
                cell_count,
                spacing,
                color,
            } => {
                gizmos.grid_2d(position, rotation, cell_count, spacing, color);
            }
            Self::Grid3d {
                position,
                rotation,
                cell_count,
                spacing,
                color,
            } => {
                gizmos.grid_3d(position, rotation, cell_count, spacing, color);
            }
            Self::Line { start, end, color } => {
                gizmos.line(start, end, color);
            }
            Self::Line2d { start, end, color } => {
                gizmos.line_2d(start, end, color);
            }
            Self::LineGradient {
                start,
                end,
                start_color,
                end_color,
            } => {
                gizmos.line_gradient(start, end, start_color, end_color);
            }
            Self::LineGradient2d {
                start,
                end,
                start_color,
                end_color,
            } => {
                gizmos.line_gradient_2d(start, end, start_color, end_color);
            }
            Self::Linestrip { positions, color } => {
                gizmos.linestrip(positions, color);
            }
            Self::Linestrip2d { positions, color } => {
                gizmos.linestrip_2d(positions, color);
            }
            Self::LinestripGradient { points } => {
                gizmos.linestrip_gradient(points);
            }
            Self::LinestripGradient2d { positions } => {
                gizmos.linestrip_gradient_2d(positions);
            }
            Self::LongArc3dBetween {
                center,
                from,
                to,
                color,
            } => {
                gizmos.long_arc_3d_between(center, from, to, color);
            }
            Self::Ray {
                start,
                vector,
                color,
            } => {
                gizmos.ray(start, vector, color);
            }
            Self::Ray2d {
                start,
                vector,
                color,
            } => {
                gizmos.ray_2d(start, vector, color);
            }
            Self::RayGradient {
                start,
                vector,
                start_color,
                end_color,
            } => {
                gizmos.ray_gradient(start, vector, start_color, end_color);
            }
            Self::RayGradient2d {
                start,
                vector,
                start_color,
                end_color,
            } => {
                gizmos.ray_gradient_2d(start, vector, start_color, end_color);
            }
            Self::Rect {
                position,
                rotation,
                size,
                color,
            } => {
                gizmos.rect(position, rotation, size, color);
            }
            Self::Rect2d {
                position,
                rotation,
                size,
                color,
            } => {
                gizmos.rect_2d(position, rotation, size, color);
            }
            Self::RoundedCuboid {
                position,
                rotation,
                size,
                color,
            } => {
                gizmos.rounded_cuboid(position, rotation, size, color);
            }
            Self::RoundedRect {
                position,
                rotation,
                size,
                color,
            } => {
                gizmos.rounded_rect(position, rotation, size, color);
            }
            Self::RoundedRect2d {
                position,
                rotation,
                size,
                color,
            } => {
                gizmos.rounded_rect_2d(position, rotation, size, color);
            }
            Self::ShortArc3dBetween {
                center,
                from,
                to,
                color,
            } => {
                gizmos.short_arc_3d_between(center, from, to, color);
            }
            Self::Sphere {
                position,
                rotation,
                radius,
                color,
            } => {
                gizmos.sphere(position, rotation, radius, color);
            }
        }
    }
}

/// Gizmo log version of [`arc_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_2d).
pub fn arc_2d(
    position: Vec2,
    direction_angle: f32,
    arc_angle: f32,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Arc2d {
        position,
        direction_angle,
        arc_angle,
        radius,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`arc_3d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_3d).
pub fn arc_3d(
    angle: f32,
    radius: f32,
    position: Vec3,
    rotation: Quat,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Arc3d {
        angle,
        radius,
        position,
        rotation,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`arrow`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow).
pub fn arrow(start: Vec3, end: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Arrow {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`arrow_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow_2d).
pub fn arrow_2d(start: Vec2, end: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Arrow2d {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`axes`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes).
pub fn axes(transform: impl IntoMat4, base_length: f32) -> String {
    ron::ser::to_string(&GizmoCommand::Axes {
        transform: transform.into_mat4(),
        base_length,
    })
    .unwrap()
}

/// Gizmo log version of [`axes_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes_2d).
pub fn axes_2d(transform: impl IntoMat4, base_length: f32) -> String {
    ron::ser::to_string(&GizmoCommand::Axes2d {
        transform: transform.into_mat4(),
        base_length,
    })
    .unwrap()
}

/// Gizmo log version of [`circle`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle).
pub fn circle(position: Vec3, normal: Dir3, radius: f32, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Circle {
        position,
        normal,
        radius,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`circle_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle_2d).
pub fn circle_2d(position: Vec2, radius: f32, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Circle2d {
        position,
        radius,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`cuboid`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.cuboid).
pub fn cuboid(transform: impl IntoMat4, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Cuboid {
        transform: transform.into_mat4(),
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ellipse`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse).
pub fn ellipse(position: Vec3, rotation: Quat, half_size: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Ellipse {
        position,
        rotation,
        half_size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ellipse_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse_2d).
pub fn ellipse_2d(position: Vec2, angle: f32, half_size: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Ellipse2d {
        position,
        angle,
        half_size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`grid`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid).
pub fn grid(
    position: Vec3,
    rotation: Quat,
    cell_count: UVec2,
    spacing: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Grid {
        position,
        rotation,
        cell_count,
        spacing,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`grid_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_2d).
pub fn grid_2d(
    position: Vec2,
    rotation: f32,
    cell_count: UVec2,
    spacing: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Grid2d {
        position,
        rotation,
        cell_count,
        spacing,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`grid_3d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_3d).
pub fn grid_3d(
    position: Vec3,
    rotation: Quat,
    cell_count: UVec3,
    spacing: Vec3,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Grid3d {
        position,
        rotation,
        cell_count,
        spacing,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`line`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line).
pub fn line(start: Vec3, end: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Line {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`line_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_2d).
pub fn line_2d(start: Vec2, end: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Line2d {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`line_gradient`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient).
pub fn line_gradient(
    start: Vec3,
    end: Vec3,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::LineGradient {
        start,
        end,
        start_color: start_color.into(),
        end_color: end_color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`line_gradient_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient_2d).
pub fn line_gradient_2d(
    start: Vec2,
    end: Vec2,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::LineGradient2d {
        start,
        end,
        start_color: start_color.into(),
        end_color: end_color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`linestrip`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip).
pub fn linestrip(positions: impl IntoIterator<Item = Vec3>, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Linestrip {
        positions: positions.into_iter().collect(),
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`linestrip_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_2d).
pub fn linestrip_2d(positions: impl IntoIterator<Item = Vec2>, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Linestrip2d {
        positions: positions.into_iter().collect(),
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`linestrip_gradient`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient).
pub fn linestrip_gradient<C: Into<Color>>(points: impl IntoIterator<Item = (Vec3, C)>) -> String {
    ron::ser::to_string(&GizmoCommand::LinestripGradient {
        points: points.into_iter().map(|(v, c)| (v, c.into())).collect(),
    })
    .unwrap()
}

/// Gizmo log version of [`linestrip_gradient_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient_2d).
pub fn linestrip_gradient_2d<C: Into<Color>>(
    positions: impl IntoIterator<Item = (Vec2, C)>,
) -> String {
    ron::ser::to_string(&GizmoCommand::LinestripGradient2d {
        positions: positions.into_iter().map(|(v, c)| (v, c.into())).collect(),
    })
    .unwrap()
}

/// Gizmo log version of [`long_arc_3d_between`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.long_arc_3d_between).
pub fn long_arc_3d_between(center: Vec3, from: Vec3, to: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::LongArc3dBetween {
        center,
        from,
        to,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ray`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray).
pub fn ray(start: Vec3, vector: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Ray {
        start,
        vector,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ray_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_2d).
pub fn ray_2d(start: Vec2, vector: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Ray2d {
        start,
        vector,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ray_gradient`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient).
pub fn ray_gradient(
    start: Vec3,
    vector: Vec3,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::RayGradient {
        start,
        vector,
        start_color: start_color.into(),
        end_color: end_color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ray_gradient_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient_2d).
pub fn ray_gradient_2d(
    start: Vec2,
    vector: Vec2,
    start_color: impl Into<Color>,
    end_color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::RayGradient2d {
        start,
        vector,
        start_color: start_color.into(),
        end_color: end_color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rect`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect).
pub fn rect(position: Vec3, rotation: Quat, size: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Rect {
        position,
        rotation,
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rect_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect_2d).
pub fn rect_2d(position: Vec2, rotation: f32, size: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Rect2d {
        position,
        rotation,
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rounded_cuboid`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_cuboid).
pub fn rounded_cuboid(
    position: Vec3,
    rotation: Quat,
    size: Vec3,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::RoundedCuboid {
        position,
        rotation,
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rounded_rect`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect).
pub fn rounded_rect(position: Vec3, rotation: Quat, size: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::RoundedRect {
        position,
        rotation,
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rounded_rect_2d`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect_2d).
pub fn rounded_rect_2d(
    position: Vec2,
    rotation: f32,
    size: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::RoundedRect2d {
        position,
        rotation,
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`short_arc_3d_between`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.short_arc_3d_between).
pub fn short_arc_3d_between(center: Vec3, from: Vec3, to: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::ShortArc3dBetween {
        center,
        from,
        to,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`sphere`](https://docs.rs/bevy/0.14.1/bevy/gizmos/gizmos/struct.Gizmos.html#method.sphere).
pub fn sphere(position: Vec3, rotation: Quat, radius: f32, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Sphere {
        position,
        rotation,
        radius,
        color: color.into(),
    })
    .unwrap()
}
