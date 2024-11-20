use bevy::prelude::{Entity, Resource};

mod errors;
mod manager;

pub use errors::TilemapManagerError;
pub use manager::TilemapManager;

/// A local resource for the tilemap manager that holds the currently selected map layer
#[derive(Resource, Default)]
pub(crate) struct LayerIndex<MapLayer>(pub(crate) MapLayer);

/// A local resource for the tilemap that holds the map entity that the tilemap manager is working with
#[derive(Default, Resource)]
pub(crate) struct MapEntity(pub(crate) Option<Entity>);
