use bevy_math::{Affine3A, Mat4};
use bevy_transform::components::{GlobalTransform, Transform};

/// Some gizmos in bevy take an `impl TransformPoint` argument
/// which can be Mat4, Affine3A, GlobalTransform, or Transform.
/// `ToMat4` is a trait that helps convert these possibilities
/// into Mat4 so that they can be serialized and logged.
pub trait IntoMat4 {
    fn into_mat4(&self) -> Mat4;
}

impl IntoMat4 for Mat4 {
    fn into_mat4(&self) -> Mat4 {
        *self
    }
}

impl IntoMat4 for Affine3A {
    fn into_mat4(&self) -> Mat4 {
        Mat4::from(*self)
    }
}

impl IntoMat4 for GlobalTransform {
    fn into_mat4(&self) -> Mat4 {
        self.compute_matrix()
    }
}

impl IntoMat4 for Transform {
    fn into_mat4(&self) -> Mat4 {
        self.compute_matrix()
    }
}
