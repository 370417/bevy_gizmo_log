use bevy_math::{Affine3A, Mat4};
#[cfg(feature = "bevy")]
use bevy_transform::components::{GlobalTransform, Transform};

/// Trait for arguments that act like transforms.
///
/// Some gizmos in bevy take an [`impl TransformPoint`] argument
/// which can be [`Mat4`], [`Affine3A`], [`GlobalTransform`], or [`Transform`].
/// [`IntoMat4`] is a trait that helps convert these possibilities
/// into [`Mat4`] so that they can be serialized and logged.
///
/// [`impl TransformPoint`]: bevy_transform::TransformPoint
pub trait IntoMat4 {
    fn into_mat4(self) -> Mat4;
}

impl IntoMat4 for Mat4 {
    fn into_mat4(self) -> Mat4 {
        self
    }
}

impl IntoMat4 for Affine3A {
    fn into_mat4(self) -> Mat4 {
        Mat4::from(self)
    }
}

#[cfg(feature = "bevy")]
#[cfg_attr(docsrs, doc(cfg(feature = "bevy")))]
impl IntoMat4 for GlobalTransform {
    fn into_mat4(self) -> Mat4 {
        self.compute_matrix()
    }
}

#[cfg(feature = "bevy")]
#[cfg_attr(docsrs, doc(cfg(feature = "bevy")))]
impl IntoMat4 for Transform {
    fn into_mat4(self) -> Mat4 {
        self.compute_matrix()
    }
}
