//! Bevy_gizmo_log gives you the ability to render gizmos by
//! logging them. This lets you render gizmos anytime, anywhere.
//!
//! ```
//! # use bevy_color::palettes::css::RED;
//! # use bevy_gizmo_log::gizmo::arrow;
//! # use bevy_log::debug;
//! # use bevy_math::Vec3;
//! debug!(gizmo = arrow(Vec3::ZERO, Vec3::ONE, RED));
//! ```
//!
//! # Compatible versions
//!
//! | bevy | bevy_gizmo_log |
//! | ---- | -------------- |
//! | 0.14 | 0.2.0,         |
//! |      | 0.1.0          |
//!
//! # Motivation
//!
//! 1. Make adding gizmos as quick and easy as possible.
//! 2. Add gizmos from anywhere, even from code not managed by bevy.
//!
//! # Getting started
//!
//! Add [`GizmoLogPlugin`] to your bevy app. If you use
//! `DefaultPlugins`, you'll also need to disable bevy's default
//! [`LogPlugin`].
//!
//! ```no_run
//! # use bevy::DefaultPlugins;
//! # use bevy_app::{App, PluginGroup};
//! # use bevy_gizmo_log::GizmoLogPlugin;
//! # use bevy_log::LogPlugin;
//! App::new()
//!     .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
//!     .add_plugins(GizmoLogPlugin::default());
//! ```
//!
//! To log a gizmo, choose a function from the [`gizmo`]
//! module and log it in a gizmo field:
//!
//! ```
//! # use bevy_color::palettes::css::RED;
//! # use bevy_log::debug;
//! # use bevy_math::Vec3;
//! # use bevy_transform::components::Transform;
//! use bevy_gizmo_log::gizmo::{arrow, axes};
//!
//! debug!(gizmo = arrow(Vec3::ZERO, Vec3::ONE, RED));
//! debug!(gizmo = axes(Transform::default(), 1.0));
//! ```
//!
//! # Feature flags
//!
//! **bevy:** Enabled by default. The bevy feature enables reading
//! gizmo logs in bevy and rendering them. You can disable this feature if
//! your crate needs to generate gizmo logs but doesn't need to consume them.
//!
//! **convert-nalgebra033:** Adds a module `gizmo_na` which lets you
//! create gizmos using nalgebra types instead of bevy's default glam types.
//!
//! [`LogPlugin`]: bevy_log::LogPlugin

// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod gizmo;

#[cfg(feature = "convert-nalgebra033")]
#[cfg_attr(docsrs, doc(cfg(feature = "convert-nalgebra033")))]
pub mod gizmo_na;

#[cfg(feature = "bevy")]
#[cfg_attr(docsrs, doc(cfg(feature = "bevy")))]
mod log_layer;
#[cfg(feature = "bevy")]
#[cfg_attr(docsrs, doc(cfg(feature = "bevy")))]
mod log_plugin;

#[cfg(feature = "bevy")]
#[cfg_attr(docsrs, doc(cfg(feature = "bevy")))]
pub use log_layer::render_gizmo_log_events;
#[cfg(feature = "bevy")]
#[cfg_attr(docsrs, doc(cfg(feature = "bevy")))]
pub use log_layer::GizmoLayer;
#[cfg(feature = "bevy")]
#[cfg_attr(docsrs, doc(cfg(feature = "bevy")))]
pub use log_plugin::GizmoLogPlugin;

mod transform;
