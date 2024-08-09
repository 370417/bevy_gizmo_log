//! Reads gizmo log events from tracing logs and sends them to a bevy resource.

use std::sync::mpsc;

use crate::command::GizmoCommand;

/// Captures log events from tracing and sends them to a bevy resource.
pub struct GizmoCaptureLayer {
    sender: mpsc::SyncSender<GizmoCommand>,
}
