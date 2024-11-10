//! Functions for logging gizmos.

use bevy_color::Color;
#[cfg(feature = "bevy")]
use bevy_gizmos::gizmos::Gizmos;
use bevy_math::{Isometry2d, Isometry3d, Mat4, UVec2, UVec3, Vec2, Vec3};

pub use crate::transform::IntoMat4;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum GizmoCommand {
    Arc2d {
        isometry: Isometry2d,
        arc_angle: f32,
        radius: f32,
        color: Color,
    },
    Arc3d {
        angle: f32,
        radius: f32,
        isometry: Isometry3d,
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
        isometry: Isometry3d,
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
        isometry: Isometry3d,
        half_size: Vec2,
        color: Color,
    },
    Ellipse2d {
        isometry: Isometry2d,
        half_size: Vec2,
        color: Color,
    },
    Grid {
        isometry: Isometry3d,
        cell_count: UVec2,
        spacing: Vec2,
        color: Color,
    },
    Grid2d {
        isometry: Isometry2d,
        cell_count: UVec2,
        spacing: Vec2,
        color: Color,
    },
    Grid3d {
        isometry: Isometry3d,
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
        isometry: Isometry3d,
        size: Vec2,
        color: Color,
    },
    Rect2d {
        isometry: Isometry2d,
        size: Vec2,
        color: Color,
    },
    RoundedCuboid {
        isometry: Isometry3d,
        size: Vec3,
        color: Color,
    },
    RoundedRect {
        isometry: Isometry3d,
        size: Vec2,
        color: Color,
    },
    RoundedRect2d {
        isometry: Isometry2d,
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
        isometry: Isometry3d,
        radius: f32,
        color: Color,
    },
}

#[cfg(feature = "bevy")]
impl GizmoCommand {
    pub fn draw(self, gizmos: &mut Gizmos) {
        match self {
            Self::Arc2d {
                isometry,
                arc_angle,
                radius,
                color,
            } => {
                gizmos.arc_2d(isometry, arc_angle, radius, color);
            }
            Self::Arc3d {
                angle,
                radius,
                isometry,
                color,
            } => {
                gizmos.arc_3d(angle, radius, isometry, color);
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
                isometry,
                radius,
                color,
            } => {
                gizmos.circle(isometry, radius, color);
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
                isometry,
                half_size,
                color,
            } => {
                gizmos.ellipse(isometry, half_size, color);
            }
            Self::Ellipse2d {
                isometry,
                half_size,
                color,
            } => {
                gizmos.ellipse_2d(isometry, half_size, color);
            }
            Self::Grid {
                isometry,
                cell_count,
                spacing,
                color,
            } => {
                gizmos.grid(isometry, cell_count, spacing, color);
            }
            Self::Grid2d {
                isometry,
                cell_count,
                spacing,
                color,
            } => {
                gizmos.grid_2d(isometry, cell_count, spacing, color);
            }
            Self::Grid3d {
                isometry,
                cell_count,
                spacing,
                color,
            } => {
                gizmos.grid_3d(isometry, cell_count, spacing, color);
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
                isometry,
                size,
                color,
            } => {
                gizmos.rect(isometry, size, color);
            }
            Self::Rect2d {
                isometry,
                size,
                color,
            } => {
                gizmos.rect_2d(isometry, size, color);
            }
            Self::RoundedCuboid {
                isometry,
                size,
                color,
            } => {
                gizmos.rounded_cuboid(isometry, size, color);
            }
            Self::RoundedRect {
                isometry,
                size,
                color,
            } => {
                gizmos.rounded_rect(isometry, size, color);
            }
            Self::RoundedRect2d {
                isometry,
                size,
                color,
            } => {
                gizmos.rounded_rect_2d(isometry, size, color);
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
                isometry,
                radius,
                color,
            } => {
                gizmos.sphere(isometry, radius, color);
            }
        }
    }
}

/// Gizmo log version of [`arc_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_2d).
pub fn arc_2d(
    isometry: impl Into<Isometry2d>,
    arc_angle: f32,
    radius: f32,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Arc2d {
        isometry: isometry.into(),
        arc_angle,
        radius,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`arc_3d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arc_3d).
pub fn arc_3d(
    angle: f32,
    radius: f32,
    isometry: impl Into<Isometry3d>,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Arc3d {
        angle,
        radius,
        isometry: isometry.into(),
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`arrow`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow).
pub fn arrow(start: Vec3, end: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Arrow {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`arrow_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.arrow_2d).
pub fn arrow_2d(start: Vec2, end: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Arrow2d {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`axes`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes).
pub fn axes(transform: impl IntoMat4, base_length: f32) -> String {
    ron::ser::to_string(&GizmoCommand::Axes {
        transform: transform.into_mat4(),
        base_length,
    })
    .unwrap()
}

/// Gizmo log version of [`axes_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.axes_2d).
pub fn axes_2d(transform: impl IntoMat4, base_length: f32) -> String {
    ron::ser::to_string(&GizmoCommand::Axes2d {
        transform: transform.into_mat4(),
        base_length,
    })
    .unwrap()
}

/// Gizmo log version of [`circle`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle).
pub fn circle(isometry: impl Into<Isometry3d>, radius: f32, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Circle {
        isometry: isometry.into(),
        radius,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`circle_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.circle_2d).
pub fn circle_2d(position: Vec2, radius: f32, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Circle2d {
        position,
        radius,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`cuboid`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.cuboid).
pub fn cuboid(transform: impl IntoMat4, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Cuboid {
        transform: transform.into_mat4(),
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ellipse`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse).
pub fn ellipse(
    isometry: impl Into<Isometry3d>,
    half_size: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Ellipse {
        isometry: isometry.into(),
        half_size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ellipse_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ellipse_2d).
pub fn ellipse_2d(
    isometry: impl Into<Isometry2d>,
    half_size: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Ellipse2d {
        isometry: isometry.into(),
        half_size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`grid`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid).
pub fn grid(
    isometry: impl Into<Isometry3d>,
    cell_count: UVec2,
    spacing: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Grid {
        isometry: isometry.into(),
        cell_count,
        spacing,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`grid_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_2d).
pub fn grid_2d(
    isometry: impl Into<Isometry2d>,
    cell_count: UVec2,
    spacing: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Grid2d {
        isometry: isometry.into(),
        cell_count,
        spacing,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`grid_3d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.grid_3d).
pub fn grid_3d(
    isometry: impl Into<Isometry3d>,
    cell_count: UVec3,
    spacing: Vec3,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::Grid3d {
        isometry: isometry.into(),
        cell_count,
        spacing,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`line`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line).
pub fn line(start: Vec3, end: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Line {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`line_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_2d).
pub fn line_2d(start: Vec2, end: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Line2d {
        start,
        end,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`line_gradient`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient).
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

/// Gizmo log version of [`line_gradient_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.line_gradient_2d).
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

/// Gizmo log version of [`linestrip`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip).
pub fn linestrip(positions: impl IntoIterator<Item = Vec3>, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Linestrip {
        positions: positions.into_iter().collect(),
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`linestrip_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_2d).
pub fn linestrip_2d(positions: impl IntoIterator<Item = Vec2>, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Linestrip2d {
        positions: positions.into_iter().collect(),
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`linestrip_gradient`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient).
pub fn linestrip_gradient<C: Into<Color>>(points: impl IntoIterator<Item = (Vec3, C)>) -> String {
    ron::ser::to_string(&GizmoCommand::LinestripGradient {
        points: points.into_iter().map(|(v, c)| (v, c.into())).collect(),
    })
    .unwrap()
}

/// Gizmo log version of [`linestrip_gradient_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.linestrip_gradient_2d).
pub fn linestrip_gradient_2d<C: Into<Color>>(
    positions: impl IntoIterator<Item = (Vec2, C)>,
) -> String {
    ron::ser::to_string(&GizmoCommand::LinestripGradient2d {
        positions: positions.into_iter().map(|(v, c)| (v, c.into())).collect(),
    })
    .unwrap()
}

/// Gizmo log version of [`long_arc_3d_between`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.long_arc_3d_between).
pub fn long_arc_3d_between(center: Vec3, from: Vec3, to: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::LongArc3dBetween {
        center,
        from,
        to,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ray`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray).
pub fn ray(start: Vec3, vector: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Ray {
        start,
        vector,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ray_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_2d).
pub fn ray_2d(start: Vec2, vector: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Ray2d {
        start,
        vector,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`ray_gradient`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient).
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

/// Gizmo log version of [`ray_gradient_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.ray_gradient_2d).
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

/// Gizmo log version of [`rect`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect).
pub fn rect(isometry: impl Into<Isometry3d>, size: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Rect {
        isometry: isometry.into(),
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rect_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rect_2d).
pub fn rect_2d(isometry: impl Into<Isometry2d>, size: Vec2, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Rect2d {
        isometry: isometry.into(),
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rounded_cuboid`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_cuboid).
pub fn rounded_cuboid(
    isometry: impl Into<Isometry3d>,
    size: Vec3,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::RoundedCuboid {
        isometry: isometry.into(),
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rounded_rect`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect).
pub fn rounded_rect(
    isometry: impl Into<Isometry3d>,
    size: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::RoundedRect {
        isometry: isometry.into(),
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`rounded_rect_2d`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.rounded_rect_2d).
pub fn rounded_rect_2d(
    isometry: impl Into<Isometry2d>,
    size: Vec2,
    color: impl Into<Color>,
) -> String {
    ron::ser::to_string(&GizmoCommand::RoundedRect2d {
        isometry: isometry.into(),
        size,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`short_arc_3d_between`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.short_arc_3d_between).
pub fn short_arc_3d_between(center: Vec3, from: Vec3, to: Vec3, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::ShortArc3dBetween {
        center,
        from,
        to,
        color: color.into(),
    })
    .unwrap()
}

/// Gizmo log version of [`sphere`](https://docs.rs/bevy/0.15.0/bevy/gizmos/gizmos/struct.Gizmos.html#method.sphere).
pub fn sphere(isometry: impl Into<Isometry3d>, radius: f32, color: impl Into<Color>) -> String {
    ron::ser::to_string(&GizmoCommand::Sphere {
        isometry: isometry.into(),
        radius,
        color: color.into(),
    })
    .unwrap()
}
