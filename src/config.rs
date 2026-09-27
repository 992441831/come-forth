use bevy::prelude::*;

/// All tunable parameters of the simulation.
#[derive(Resource, Clone)]
pub struct SimConfig {
    /// Half-width and half-height of the simulation world.
    pub world_half_size: Vec2,

    pub creature_count: usize,
    pub food_count: usize,

    /// Simulation ticks per second.
    pub ticks_per_second: f32,

    /// Energy lost each tick just by living.
    pub metabolism_rate: f32,
    /// Energy gained from eating one food particle.
    pub food_energy_value: f32,
    /// Threshold below which a creature is considered hungry and actively seeks food.
    pub hungry_threshold: f32,

    /// Minimum energy required to reproduce.
    pub reproduction_threshold: f32,
    /// Energy subtracted from parent (and given in part to child).
    pub reproduction_cost: f32,
    /// Ticks between reproduction attempts.
    pub reproduction_cooldown: f32,

    /// Maximum speed in world units per tick.
    pub max_speed: f32,
    /// Steering force multiplier.
    pub seek_strength: f32,
    /// Wandering strength when not seeking.
    pub wander_strength: f32,

    /// How far a creature can see food.
    pub perception_radius: f32,
    /// How far a creature can communicate with same-species neighbors.
    pub communication_radius: f32,

    /// Food respawned per tick.
    pub food_spawn_per_tick: usize,

    /// Number of species. Each gets a distinct color and only communicates within itself.
    pub species_count: u8,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            world_half_size: Vec2::new(1200.0, 800.0),
            creature_count: 10_000,
            food_count: 1_500,
            ticks_per_second: 30.0,
            metabolism_rate: 0.12,
            food_energy_value: 25.0,
            hungry_threshold: 0.55,
            reproduction_threshold: 75.0,
            reproduction_cost: 35.0,
            reproduction_cooldown: 20.0,
            max_speed: 6.0,
            seek_strength: 0.35,
            wander_strength: 0.12,
            perception_radius: 70.0,
            communication_radius: 50.0,
            food_spawn_per_tick: 40,
            species_count: 3,
        }
    }
}

impl SimConfig {
    /// Cell size for the uniform spatial grid.
    /// Must be at least as large as the largest interaction radius
    /// so any neighbor lives in the same or an adjacent cell.
    pub fn cell_size(&self) -> f32 {
        self.perception_radius.max(self.communication_radius) * 1.5
    }
}
