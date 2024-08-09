# bevy_gizmo_log

Bevy_gizmo_log gives you the ability to render gizmos by
logging them. This lets you render gizmos anytime, anywhere.

```rust
debug!(gizmo = arrow(Vec3::ZERO, Vec3::ONE, RED));
```

## Compatible versions

| bevy | bevy_gizmo_log |
| ---- | -------------- |
| 0.14 | 0.1            |

## Motivation

1.  Make adding gizmos as quick and easy as possible.
2.  Add gizmos from anywhere, even from code not managed by bevy.

## Getting started

Add `GizmoLogPlugin` to your bevy app. If you use
`DefaultPlugins`, you'll also need to disable bevy's default
`LogPlugin`.

```rust
App::new()
    .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
    .add_plugins(GizmoLogPlugin::default());
```

To log a gizmo, choose a function from the `gizmo`
module and log it in a gizmo field:

```rust
use bevy_gizmo_log::gizmo::{arrow, axes};

debug!(gizmo = arrow(Vec3::ZERO, Vec3::ONE, RED));
debug!(gizmo = axes(Transform::default(), 1.0));
```

## Feature flags

**bevy:** Enabled by default. The bevy feature enables reading
gizmo logs in bevy and rendering them. You can disable this feature if
your crate needs to generate gizmo logs but doesn't need to consume them.

**convert-nalgebra033:** Adds a module `gizmo_na` which lets you
create gizmos using nalgebra types instead of bevy's default glam types.
