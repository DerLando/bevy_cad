mod camera;
mod primitives;
mod attributes;
mod drawing_object;
pub(crate) mod render;

use attributes::name::Name;
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use camera::PanOrbitCameraPlugin;
use drawing_object::{DrawingObject, DrawingObjectBundle, GeometryType};
use primitives::{Primitive, cuboid::{Cuboid, CuboidBundle}};
use render::RenderMesh;

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app
        .add_startup_system(setup.system())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PanOrbitCameraPlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    //     ..Default::default()
    // });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    PanOrbitCameraPlugin::spawn_camera(
        &mut commands, 
    Vec3::new(-2.0, 2.5, 5.0),
    Vec3::ZERO, 
    Vec3::Y);

    let cuboid = Cuboid::new_bundle(2.0, 1.0, 0.5);
    let mesh = cuboid.create_render_mesh();
    commands.spawn_bundle(DrawingObjectBundle {
        object: DrawingObject,
        name: Name(String::from("test")),
        geometry_type: GeometryType::Primitive(Primitive::Cuboid(cuboid)),
        render_bundle: PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        }
    });
}
