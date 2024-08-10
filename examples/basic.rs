use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_gizmo_log::{
    gizmo::{circle_2d, linestrip_gradient_2d},
    GizmoLogPlugin,
};
use bevy_log::LogPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(GizmoLogPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update() {
    // Log a gizmo inline
    debug!(gizmo = circle_2d(Vec2::ZERO, 256., WHITE));

    for i in 1..250 {
        let i = i as f32;
        let gizmo = linestrip_gradient_2d([
            vertex(0.00, i / 250.),
            vertex(60.0, i / 250.),
            vertex(120., i / 250.),
            vertex(180., i / 250.),
            vertex(240., i / 250.),
            vertex(300., i / 250.),
            vertex(360., i / 250.),
        ]);
        // Or log a gizmo using shorthand
        debug!(gizmo);
    }
}

fn vertex(degrees: f32, radius: f32) -> (Vec2, Hsva) {
    (
        Rot2::from(degrees.to_radians()) * Vec2::X * 250. * radius,
        Hsva::new(degrees, radius, 1., 1.),
    )
}
