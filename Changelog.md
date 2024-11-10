# v0.3.0

- Updated bevy to v0.15

# v0.2.0

- Moved `render_gizmo_log_events` from the `Update` schedule to the `PostUpdate` schedule. This avoids flickering when logging gizmos from an `Update` system.
- Made `GizmoLogEventReceiver` public so that `render_gizmo_log_events` can be referenced.

# v0.1.0

Initial release.
