use bevy::prelude::*;

fn main() {
    // bevy general
    App::new().add_plugins(DefaultPlugins).add_systems(Startup,
         (spawn_camera, 
            spawn_basic_scene,
            print_time,)).run();

}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y)),
        material: materials.add(Color::MIDNIGHT_BLUE),..default()
    });
}

fn print_time(time: Res<Time>){
    println!("{}", time.delta_seconds());

}

