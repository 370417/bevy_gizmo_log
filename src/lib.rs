mod bevy;
mod command;
mod tracing;

use bevy_app::App;
use bevy_log::BoxedLayer;

/// ```
/// # use bevy_app::{App, PluginGroup};
/// # use bevy_internal::DefaultPlugins;
/// # use bevy_log::LogPlugin;
/// # use bevy_gizmo_log::gizmo_log_layer;
/// App::new().add_plugins(
///     DefaultPlugins.set(LogPlugin {
///         custom_layer: gizmo_log_layer,
///         ..Default::default()
///     })
/// );
/// ```
pub fn gizmo_log_layer(_app: &mut App) -> Option<BoxedLayer> {
    None
}
