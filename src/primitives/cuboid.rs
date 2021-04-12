use bevy::{pbr::PbrBundle, prelude::{Bundle, Mesh, Transform, shape}};

use crate::render::RenderMesh;

use super::dimension::{Height, Length, Width};

pub(crate) struct Cuboid;

impl Cuboid {
    pub(crate) fn new_bundle(x: f32, y: f32, z: f32) -> CuboidBundle {
        CuboidBundle {
            cube: Cuboid,
            length: Length(x),
            width: Width(y),
            height: Height(z),
        }
    }
}

#[derive(Bundle)]
pub(crate) struct CuboidBundle {
    cube: Cuboid,
    length: Length,
    width: Width,
    height: Height,
}

impl From<&CuboidBundle> for shape::Box {
    fn from(cuboid: &CuboidBundle) -> Self {
        let min_x = cuboid.length.0 / -2.0;
        let max_x = cuboid.length.0 / 2.0;
        let min_y = cuboid.width.0 / -2.0;
        let max_y = cuboid.width.0 / 2.0;
        let min_z = cuboid.height.0 / -2.0;
        let max_z = cuboid.height.0 / 2.0;

        shape::Box{
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z
        }
    }
}

impl Default for CuboidBundle {
    fn default() -> Self {
        CuboidBundle {
            cube: Cuboid,
            length: Length(1.0),
            width: Width(1.0),
            height: Height(1.0)
        }
    }
}

impl RenderMesh for CuboidBundle {
    fn create_render_mesh(&self) -> Mesh {
        Mesh::from(shape::Box::from(self))
    }
}