use std::f32::consts::PI;

use bevy::{
    ecs::{entity, reflect, system::EntityCommands},
    prelude::*,
};

fn main() {
    // bevy general
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
        ))
        .add_systems(
            Startup,
            (spawn_camera, spawn_basic_scene, print_time, asset_loading),
        )
        .add_systems(Update, (tower_shooting, bullet_despawn))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::ONE)),
        material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 1.0, 0.2)),
            material: materials.add(Color::srgb(1.0, 0.0, 1.0)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        Name::new("Tower"),
    ));
}

fn print_time(time: Res<Time>) {
    println!("{}", time.delta_seconds());
}

fn tower_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands.spawn((
                SceneBundle {
                    scene: bullet_assets.bullet_scene.clone_weak(),
                    transform: spawn_transform,
                    ..Default::default()
                },
                Lifetime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
                Name::new("Bullet"),
            ));
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load(GltfAssetLabel::Scene(0).from_asset("bullet.glb")),
    });
}

#[derive(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}
