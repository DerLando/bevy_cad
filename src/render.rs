use bevy::prelude::Mesh;

pub(crate) trait RenderMesh {
    fn create_render_mesh(&self) -> Mesh;
}