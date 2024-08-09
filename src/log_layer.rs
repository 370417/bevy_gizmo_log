//! Reads gizmo log events from tracing logs and sends them to a bevy resource.

use std::sync::mpsc;

use bevy_app::{App, Update};
use bevy_ecs::system::NonSend;
use bevy_gizmos::gizmos::Gizmos;
use bevy_log::tracing_subscriber::{layer::Context, Layer};
use bevy_utils::tracing::{
    field::{Field, Visit},
    Event, Subscriber,
};

use crate::gizmo::GizmoCommand;

/// A tracing_subscriber::Layer that handles gizmo logs.
pub struct GizmoLayer {
    sender: mpsc::Sender<GizmoCommand>,
}

impl GizmoLayer {
    pub fn new(app: &mut App) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        app.insert_non_send_resource(GizmoLogEventReceiver(receiver));
        app.add_systems(Update, render_gizmo_log_events);
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
struct GizmoLogEventReceiver(pub mpsc::Receiver<GizmoCommand>);

fn render_gizmo_log_events(receiver: NonSend<GizmoLogEventReceiver>, mut gizmos: Gizmos) {
    for gizmo_command in receiver.0.try_iter() {
        gizmo_command.draw(&mut gizmos);
    }
}
