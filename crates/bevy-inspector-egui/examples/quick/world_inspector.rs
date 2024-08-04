use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs::entity::EntityHashSet;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .register_type::<EntitiesSet>()
        .register_type::<CoolSet>()
        .run();
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct EntitiesSet(pub EntityHashSet);

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct CoolSet(pub EntityHashSet);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    let e1 = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        })
        .id();
    // cube
    let e2 = commands
        .spawn((
            Name::new("My Cube"),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
        ))
        .id();
    // light
    let e3 = commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 2_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .id();
    // camera
    let e4 = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    commands.spawn((
        Name::new("All Entities"),
        EntitiesSet(EntityHashSet::from_iter([e1, e2, e3, e4])),
        CoolSet(EntityHashSet::from_iter([e2, e3])),
    ));
}
