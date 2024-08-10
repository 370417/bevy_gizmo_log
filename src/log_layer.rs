use std::sync::mpsc;

use bevy_app::App;
use bevy_ecs::{schedule::ScheduleLabel, system::NonSend};
use bevy_gizmos::gizmos::Gizmos;
use tracing::{
    field::{Field, Visit},
    Event, Subscriber,
};
use tracing_subscriber::{layer::Context, Layer};

use crate::gizmo::GizmoCommand;

/// A [`tracing_subscriber::Layer`] that handles gizmo logs.
///
/// For use when you don't want to use [`GizmoLogPlugin`].
///
/// [`GizmoLogPlugin`]: crate::GizmoLogPlugin
pub struct GizmoLayer {
    sender: mpsc::Sender<GizmoCommand>,
}

impl GizmoLayer {
    /// Create a new [`GizmoLayer`] and setup `app` to render gizmos from logs.
    ///
    /// `schedule` is the schedule used by [`render_gizmo_log_events`] to render gizmos.
    pub fn new(app: &mut App, schedule: impl ScheduleLabel) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        app.insert_non_send_resource(GizmoLogEventReceiver(receiver));
        app.add_systems(schedule, render_gizmo_log_events);
        GizmoLayer { sender }
    }
}

impl<S: Subscriber> Layer<S> for GizmoLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        if let Some(gizmo_command) = extract_gizmo_command(event) {
            let _ = self.sender.send(gizmo_command);
        }
    }
}

/// Visitor that extracts the gizmo field of an event into a GizmoCommand.
struct GizmoVisitor(Option<GizmoCommand>);

impl Visit for GizmoVisitor {
    fn record_debug(&mut self, _field: &Field, _value: &dyn std::fmt::Debug) {
        // Do nothing. We only support extracting from strings.
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "gizmo" {
            if let Ok(gizmo_command) = ron::de::from_str(value) {
                self.0 = Some(gizmo_command);
            }
        }
    }
}

/// Convenience function for creating a one-off visitor and using it on one event.
fn extract_gizmo_command(event: &Event<'_>) -> Option<GizmoCommand> {
    let mut visitor = GizmoVisitor(None);
    event.record(&mut visitor);
    visitor.0
}

/// Bevy non-send resource that receives gizmo log events.
pub struct GizmoLogEventReceiver(mpsc::Receiver<GizmoCommand>);

/// Bevy system that ultimately renders the gizmos.
///
/// This system is made public in case you want to set other
/// systems to run before or after it.
/// There should be no need to add this system manually.
///
/// By default, runs in [`PostUpdate`].
///
/// [`PostUpdate`]: bevy_app::PostUpdate
pub fn render_gizmo_log_events(receiver: NonSend<GizmoLogEventReceiver>, mut gizmos: Gizmos) {
    for gizmo_command in receiver.0.try_iter() {
        gizmo_command.draw(&mut gizmos);
    }
}
