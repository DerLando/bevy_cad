
use crate::render::RenderMesh;

use self::cuboid::{Cuboid, CuboidBundle};

pub(crate) mod cuboid;
pub(crate) mod dimension;

#[non_exhaustive]
pub(crate) enum Primitive {
    Cuboid(CuboidBundle)
}

impl RenderMesh for Primitive {
    fn create_render_mesh(&self) -> bevy::prelude::Mesh {
        match self {
            Primitive::Cuboid(c) => c.create_render_mesh(),
            _ => unreachable!()
        }
    }
}