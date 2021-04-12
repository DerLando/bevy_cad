use bevy::prelude::{Bundle, ColorMaterial, StandardMaterial};

use super::name::Name;

pub(crate) struct Layer;

#[derive(Bundle)]
pub(crate) struct LayerBundle {
    layer: Layer,
    name: Name,
    // material: StandardMaterial
}