use bevy::prelude::*;
use std::collections::HashMap;

use crate::components::{Creature, Food, Position};
use crate::config::SimConfig;

/// Uniform grid acceleration structure for nearby-entity queries.
#[derive(Resource, Default)]
pub struct SpatialGrid {
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
    pub cell_size: f32,
}

impl SpatialGrid {
    pub fn clear(&mut self) {
        // Clear each bucket to keep its allocated capacity.
        for bucket in self.cells.values_mut() {
            bucket.clear();
        }
        self.cells.clear();
    }

    pub fn insert(&mut self, entity: Entity, pos: Vec2) {
        let cell = pos_to_cell(pos, self.cell_size);
        self.cells.entry(cell).or_default().push(entity);
    }
}

/// Convert a world position to a grid cell coordinate.
pub fn pos_to_cell(pos: Vec2, cell_size: f32) -> (i32, i32) {
    (
        (pos.x / cell_size).floor() as i32,
        (pos.y / cell_size).floor() as i32,
    )
}

/// Fill `out` with all entity IDs whose cells overlap the query circle.
/// `out` is not cleared; caller should do so.
pub fn nearby_entities(
    grid: &SpatialGrid,
    pos: Vec2,
    radius: f32,
    out: &mut Vec<Entity>,
) {
    let cell = pos_to_cell(pos, grid.cell_size);
    let range = (radius / grid.cell_size).ceil() as i32;

    for dx in -range..=range {
        for dy in -range..=range {
            if let Some(bucket) = grid.cells.get(&(cell.0 + dx, cell.1 + dy)) {
                out.extend(bucket.iter().copied());
            }
        }
    }
}

/// Rebuild the spatial grid from current creature and food positions.
pub fn update_spatial_grid(
    mut grid: ResMut<SpatialGrid>,
    config: Res<SimConfig>,
    query: Query<(Entity, &Position), Or<(With<Creature>, With<Food>)>>,
) {
    grid.cell_size = config.cell_size();
    grid.clear();

    for (entity, pos) in query.iter() {
        grid.insert(entity, pos.0);
    }
}
