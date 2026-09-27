use bevy::prelude::*;
use std::collections::VecDeque;

/// 模拟 tick 计数器。
///
/// 每进行一次 FixedUpdate 逻辑更新，该计数器加 1。
/// 用于在 UI 上显示模拟已经运行了多少轮。
#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct SimTick {
    pub count: u64,
}

/// 生物数量与食物数量的历史数据，用于绘制折线图。
///
/// 每个 tick 记录一次原始数值，渲染时按秒采样或聚合。
/// 默认保留最近 60 秒的数据（配合每秒一次的采样频率）。
#[derive(Resource, Default, Debug, Clone)]
pub struct PopulationHistory {
    /// 每个采样时刻的生物数量。
    pub creature_counts: VecDeque<f32>,
    /// 每个采样时刻的食物数量。
    pub food_counts: VecDeque<f32>,
    /// 每个采样时刻的捕食者数量。
    pub predator_counts: VecDeque<f32>,
}

/// 折线图数据采样定时器。
///
/// 控制每隔多久往 PopulationHistory 里写入一个数据点。
#[derive(Resource, Debug, Clone)]
pub struct ChartUpdateTimer {
    pub timer: Timer,
}

/// Marker for living creatures.
#[derive(Component)]
pub struct Creature;

/// Marker for food particles.
#[derive(Component)]
pub struct Food;

/// Marker for predators that hunt creatures.
#[derive(Component)]
pub struct Predator;

/// Marker added to a creature when it detects a nearby predator and is fleeing.
#[derive(Component)]
pub struct Fleeing;

/// Marker for lake entities.
#[derive(Component)]
pub struct Lake;

/// Tracks how much longer a creature can stay inside a lake before drowning.
#[derive(Component, Debug, Clone, Copy)]
pub struct InLake {
    /// Remaining safe time in the lake, in seconds.
    pub time_remaining: f32,
}

/// 2D position in world space.
#[derive(Component, Clone, Copy)]
pub struct Position(pub Vec2);

/// 2D velocity.
#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec2);

/// Energy pool. When it hits zero the creature starves.
#[derive(Component, Clone, Copy)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
}

/// Age in simulation ticks.
#[derive(Component, Clone, Copy)]
pub struct Age {
    pub ticks: u32,
    pub max_lifetime: u32,
}

/// Species identifier and display color.
#[derive(Component, Clone, Copy)]
pub struct Species {
    pub id: u8,
    pub color: Color,
}

/// A message passed between creatures of the same species.
#[derive(Clone, Copy)]
pub struct Message {
    pub sender_pos: Vec2,
    /// Normalized direction from sender toward known food, if any.
    pub food_direction: Option<Vec2>,
    pub sender_energy: f32,
}

/// Stores the most recent message received from a neighbor.
#[derive(Component, Default)]
pub struct CommunicationBuffer {
    pub last_message: Option<Message>,
}

/// Cooldown timer to prevent instant population explosions.
#[derive(Component, Clone, Copy)]
pub struct ReproductionCooldown {
    pub timer: f32,
}

/// Event: request to spawn a new creature.
#[derive(Event)]
pub struct SpawnCreature {
    pub pos: Vec2,
    pub species_id: u8,
    pub energy: f32,
}

/// Event: request to spawn a new predator.
#[derive(Event)]
pub struct SpawnPredator {
    pub pos: Vec2,
    pub energy: f32,
}

/// Event: request to spawn a food particle.
#[derive(Event)]
pub struct SpawnFood {
    pub pos: Vec2,
}

/// Event: an entity should die.
#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
    pub reason: DeathReason,
}

#[derive(Clone, Copy, Debug)]
pub enum DeathReason {
    Starvation,
    OldAge,
    Predation,
    Drowning,
}

/// Event: creature A wants to tell creature B about food.
#[derive(Event)]
pub struct CommunicationEvent {
    pub receiver: Entity,
    pub message: Message,
}
