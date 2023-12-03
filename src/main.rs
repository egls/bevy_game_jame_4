// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroid Juggler".to_string().into(),
                resolution: (1920., 1080.).into(),
                ..Default::default()
            }),
            ..default()
        }))
        //.add_systems(Startup, startup_screen)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("planets/planet00.png"),
        transform: Transform::from_scale(Vec3::splat(0.15)),
        ..Default::default()
    });
}

fn startup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("start_screen.png"),
        ..Default::default()
    });
}
