use bevy_app::{App, Plugin};
use bevy_log::{BoxedLayer, Level};
use tracing::{self, Subscriber};
use tracing_log::LogTracer;
use tracing_subscriber::{filter::FilterFn, fmt, layer::SubscriberExt, EnvFilter, Layer, Registry};

use crate::log_layer::GizmoLayer;

/// Replacement for bevy's `LogPlugin` that handles gizmo logs.
///
/// If you use `DefaultPlugins`, make sure to disable `LogPlugin`.
/// `LogPlugin` and `GizmoLogPlugin` both create tracing subscribers.
/// Only one tracing subscriber can be used at a time.
///
/// ```
/// # use bevy_app::{App, PluginGroup};
/// # use bevy_internal::DefaultPlugins;
/// # use bevy_gizmo_log::GizmoLogPlugin;
/// # use bevy_log::LogPlugin;
/// App::new()
///     .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
///     .add_plugins(GizmoLogPlugin::default());
/// ```
///
/// `GizmoLogPlugin` acts a drop-in replacement for `LogPlugin` for simple usecases.
/// It currently does not support certain bevy features (trace, tracing-chrome,
/// and tracing-tracy) or alternate targets (wasm32 and android).
///
/// If you don't want to use the entire `GizmoLogPlugin`, you can use `GizmoLayer`
/// with your own tracing subscriber to handle gizmo logs.
pub struct GizmoLogPlugin {
    // We cannot use bevy's LogPlugin and it's custom_layer field
    // because bevy's LogPlugin can only filter globally. We want
    // GizmoLayer to receive gizmo logs so that they can
    // be rendered, but we don't want to pollute stderr with gizmzo
    // logs every frame.
    pub filter: String,
    pub level: Level,
    pub custom_layer: fn(app: &mut App) -> Option<BoxedLayer>,
}

impl Default for GizmoLogPlugin {
    fn default() -> Self {
        Self {
            // Same default as bevy's LogPlugin plus an exception
            // for gizmo fields
            filter: "wgpu=error,naga=warn,[{gizmo}]=debug".to_owned(),
            level: Level::INFO,
            custom_layer: |_| None,
        }
    }
}

impl Plugin for GizmoLogPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let subscriber = Registry::default();

        let to_stderr_layer = fmt::Layer::default().with_writer(std::io::stderr);
        // Filter out gizmo logs from stderr
        // TODO: filter by log level, not by field?
        let to_stderr_layer = to_stderr_layer
            .with_filter(FilterFn::new(|meta| meta.fields().field("gizmo").is_none()));

        let subscriber = subscriber
            .with((self.custom_layer)(app))
            .with(GizmoLayer::new(app))
            .with(self.global_filter_layer())
            .with(to_stderr_layer);

        Self::set_global_subscriber(subscriber);
    }
}

impl GizmoLogPlugin {
    fn global_filter_layer(&self) -> EnvFilter {
        let default_filter = { format!("{},{}", self.level, self.filter) };
        EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(&default_filter))
            .unwrap()
    }

    fn set_global_subscriber<S: Subscriber + Send + Sync + 'static>(subscriber: S) {
        let logger_already_set = LogTracer::init().is_err();
        let subscriber_already_set = tracing::subscriber::set_global_default(subscriber).is_err();

        match (logger_already_set, subscriber_already_set) {
            (true, true) => tracing::error!(
                "Could not set global logger and tracing subscriber as they are already set."
            ),
            (true, false) => tracing::error!("Could not set global logger as it is already set."),
            (false, true) => {
                tracing::error!("Could not set global tracing subscriber as it is already set.")
            }
            (false, false) => (),
        }
    }
}
