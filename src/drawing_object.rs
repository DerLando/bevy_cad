use bevy::{pbr::PbrBundle, prelude::{Bundle, ColorMaterial, Handle, Mesh, StandardMaterial, Transform}};

use crate::{attributes::{layer::Layer, name::Name}, primitives::Primitive, render::RenderMesh};

#[non_exhaustive]
pub(crate) enum GeometryType {
    Primitive(Primitive)
}

impl RenderMesh for GeometryType {
    fn create_render_mesh(&self) -> Mesh {
        match self {
            GeometryType::Primitive(p) => p.create_render_mesh(),
            _ => unreachable!()
        }
    }
}

pub(crate) struct DrawingObject;

#[derive(Bundle)]
pub(crate) struct DrawingObjectBundle {
    pub(crate) object: DrawingObject,
    pub(crate) name: Name,
    // layer: Layer,
    pub(crate) geometry_type: GeometryType,

    #[bundle]
    pub(crate) render_bundle: PbrBundle,
}