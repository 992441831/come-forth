use bevy::prelude::*;

/// 模拟世界的全局可调参数。
///
/// 所有数值都在这里集中管理，修改后重新运行即可生效。
/// 这些参数共同决定了生物种群的动态平衡：食物、繁殖、死亡之间的节奏。
#[derive(Resource, Clone)]
pub struct SimConfig {
    /// 世界半宽与半高。
    ///
    /// 实际世界范围是 `[-world_half_size.x, world_half_size.x]`
    /// 与 `[-world_half_size.y, world_half_size.y]` 的矩形区域。
    /// 生物和食物都会在这个范围内活动。
    pub world_half_size: Vec2,

    /// 初始生物数量。
    ///
    /// 启动时会在世界范围内随机位置生成这么多生物。
    pub creature_count: usize,

    /// 初始食物数量。
    ///
    /// 启动时会在世界范围内随机位置生成这么多食物粒子。
    pub food_count: usize,

    /// 模拟 tick 频率，单位：次/秒。
    ///
    /// 所有与生物行为、代谢、繁殖、食物投放相关的系统都在 FixedUpdate 中运行，
    /// 因此这个值决定了模拟逻辑更新的速度。
    /// 例如 30.0 表示每秒 30 个 tick，每个 tick 约 33.3 毫秒。
    pub ticks_per_second: f32,

    /// 每 tick 基础代谢消耗的能量。
    ///
    /// 只要活着，每个生物每 tick 都会扣除这么多能量。
    /// 数值越大，生物饿死得越快，种群压力越大。
    pub metabolism_rate: f32,

    /// 吃掉一份食物后恢复的能量。
    ///
    /// 生物进入食物附近时会自动吃掉它，并将当前能量增加该值（上限为 energy.max）。
    pub food_energy_value: f32,

    /// 饥饿阈值，以能量百分比表示。
    ///
    /// 当 `current_energy / max_energy` 低于这个值时，生物会主动寻找食物，
    /// 并且会与同族分享食物位置信息。
    /// 例如 0.55 表示能量低于 55% 时进入饥饿状态。
    pub hungry_threshold: f32,

    /// 繁殖所需的最低能量。
    ///
    /// 只有当前能量大于等于这个值的生物才可能繁殖。
    pub reproduction_threshold: f32,

    /// 繁殖时从父代扣除的能量，也是后代初始能量的基础。
    ///
    /// 后代实际获得的能量是 `reproduction_cost * 0.5`。
    /// 父代能量会减少 `reproduction_cost`。
    pub reproduction_cost: f32,

    /// 繁殖冷却时间，单位：tick。
    ///
    /// 生物成功繁殖后，需要等待这么多 tick 才能再次繁殖。
    /// 用于防止种群在食物充足时瞬间爆炸。
    pub reproduction_cooldown: f32,

    /// 生物最大移动速度，单位：世界单位/ tick。
    ///
    /// 限制了生物每 tick 最多能移动多远。
    pub max_speed: f32,

    /// 觅食时的转向力度。
    ///
    /// 数值越大，生物朝向食物或接收到的食物方向转向越激进。
    pub seek_strength: f32,

    /// 漫游时的随机转向力度。
    ///
    /// 即使没有目标，生物也会以这个强度随机改变方向，避免全部静止或扎堆。
    pub wander_strength: f32,

    /// 感知半径，单位：世界单位。
    ///
    /// 饥饿时生物只会在这个范围内寻找食物。
    /// 数值越大，生物“看得越远”。
    pub perception_radius: f32,

    /// 通讯半径，单位：世界单位。
    ///
    /// 饥饿且发现食物的生物会把这个信息告诉同族，但只在这个半径内的同族能收到。
    pub communication_radius: f32,

    /// 每个 tick 投放的食物粒子数量。
    ///
    /// 结合 `ticks_per_second`，可以算出每秒投放总量。
    /// 例如 40 × 30 = 每秒 1200 个食物。
    pub food_spawn_per_tick: usize,

    /// 物种数量。
    ///
    /// 每个物种拥有独立的颜色，并且只在同种之间通讯。
    /// 颜色循环为：红 → 蓝 → 黄 → 绿。
    pub species_count: u8,

    /// 初始捕食者数量。
    ///
    /// 捕食者会追逐并吃掉普通生物，自身也需要能量维持生存。
    /// 它们不参与同族通讯，是食物链顶层的独立角色。
    pub predator_count: usize,

    /// 捕食者最大移动速度。
    ///
    /// 通常应略高于普通生物，否则永远追不上猎物。
    pub predator_speed: f32,

    /// 捕食者感知半径。
    ///
    /// 捕食者只在这个范围内探测普通生物并发起追击。
    pub predator_perception_radius: f32,

    /// 捕食者每 tick 代谢消耗的能量。
    ///
    /// 数值越大，捕食者越容易饿死，对猎物种群压力越小。
    pub predator_metabolism_rate: f32,

    /// 捕食者吃掉一只生物后获得的能量。
    ///
    /// 这是捕食者主要的能量来源。
    pub predator_energy_gain: f32,

    /// 捕食者繁殖所需的最低能量。
    ///
    /// 能量充足时捕食者会产生后代，维持捕食者种群。
    pub predator_reproduction_threshold: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            // 世界尺寸：实际范围 2400 × 1600
            world_half_size: Vec2::new(1200.0, 800.0),

            // 初始种群规模
            creature_count: 10_000,
            food_count: 1_500,

            // 逻辑更新频率：每秒 30 tick
            ticks_per_second: 30.0,

            // 能量相关
            metabolism_rate: 0.12,
            food_energy_value: 25.0,
            hungry_threshold: 0.55,

            // 繁殖相关
            reproduction_threshold: 75.0,
            reproduction_cost: 35.0,
            reproduction_cooldown: 20.0,

            // 移动与行为
            max_speed: 6.0,
            seek_strength: 0.35,
            wander_strength: 0.12,
            perception_radius: 70.0,
            communication_radius: 50.0,

            // 食物投放：每 tick 40 个，每秒 1200 个
            food_spawn_per_tick: 40,

            // 物种数量
            species_count: 3,

            // 捕食者相关
            predator_count: 20,
            predator_speed: 7.5,
            predator_perception_radius: 120.0,
            predator_metabolism_rate: 0.25,
            predator_energy_gain: 60.0,
            predator_reproduction_threshold: 90.0,
        }
    }
}

impl SimConfig {
    /// 空间网格的单元格大小。
    ///
    /// 必须至少不小于最大的交互半径（感知半径或通讯半径），
    /// 这样任意两个可能发生交互的生物一定位于同一个或相邻的单元格中，
    /// 从而保证邻居查询的正确性。
    /// 这里取最大半径的 1.5 倍，兼顾查询效率与正确性。
    pub fn cell_size(&self) -> f32 {
        self.perception_radius
            .max(self.communication_radius)
            .max(self.predator_perception_radius)
            * 1.5
    }
}
