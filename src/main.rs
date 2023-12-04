// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroid Juggler".to_string(),
                resolution: (1920., 1080.).into(),
                ..Default::default()
            }),
            ..default()
        }))
        //.add_systems(Startup, startup_screen)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_asteroid)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

//fn startup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
//    commands.spawn(SpriteBundle {
//        texture: asset_server.load("start_screen.png"),
//        ..Default::default()
//    });
//}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("planets/planet00.png"),
        transform: Transform::from_scale(Vec3::splat(0.15)),
        ..Default::default()
    });
}

fn spawn_asteroid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..1000 {
        let x = random::<f32>() * window.width() - window.width() / 2.0;
        let y = random::<f32>() * window.height() - window.height() / 2.0;

        commands.spawn(SpriteBundle {
            texture: asset_server.load("asteroids/meteorGrey_tiny1.png"),
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        });
    }

    //commands.spawn(SpriteBundle {
    //    texture: asset_server.load("asteroids/asteroid00.png"),
    //    transform: Transform::from_scale(Vec3::splat(0.15)),
    //    ..Default::default()
    //    });
}
