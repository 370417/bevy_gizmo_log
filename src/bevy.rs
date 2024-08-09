//! Receives gizmo log events through a queue and calls the Gizmo api.

use std::sync::mpsc;

/// Bevy non-send resource that receives gizmo log events.
pub struct GizmoLogEventReceiver(mpsc::Receiver<()>);
