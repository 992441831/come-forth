use bevy::prelude::*;
use std::collections::VecDeque;

use crate::components::{ChartUpdateTimer, Creature, Food, PopulationHistory};

/// 折线图在屏幕上的尺寸与边距，单位：像素。
///
/// 这些值会在渲染时根据当前相机的缩放比例转换为世界单位，
/// 从而让折线图在屏幕上保持固定大小和位置，不随滚轮缩放而移动或变形。
const CHART_WIDTH_PX: f32 = 260.0;
const CHART_HEIGHT_PX: f32 = 70.0;
const CHART_MARGIN_PX: f32 = 12.0;
const CHART_GAP_PX: f32 = 8.0;
const MAX_SECONDS: usize = 60;

/// 初始化历史数据与采样定时器。
pub fn setup_history(mut commands: Commands) {
    commands.insert_resource(PopulationHistory::default());
    commands.insert_resource(ChartUpdateTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}

/// 每秒采样一次当前的生物数量与食物数量，存入历史数据。
pub fn sample_population(
    creatures: Query<(), With<Creature>>,
    food: Query<(), With<Food>>,
    mut history: ResMut<PopulationHistory>,
    mut timer: ResMut<ChartUpdateTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.just_finished() {
        return;
    }

    history
        .creature_counts
        .push_back(creatures.iter().count() as f32);
    history.food_counts.push_back(food.iter().count() as f32);

    while history.creature_counts.len() > MAX_SECONDS {
        history.creature_counts.pop_front();
    }
    while history.food_counts.len() > MAX_SECONDS {
        history.food_counts.pop_front();
    }
}

/// 在屏幕右上角绘制生物数量与食物数量的折线图。
///
/// 为了避免双 Y 轴（两个量纲差距过大），采用上下两个独立的小折线图：
/// - 上方：生物数量（蓝色）
/// - 下方：食物数量（绿色）
///
/// 每个小图使用自己的历史最大值作为上限，从而清晰展示相对波动。
///
/// 图表大小和位置以像素为基准，再根据当前相机的 viewport 与投影范围
/// 换算成世界单位，因此滚轮缩放时图表在屏幕上保持完全静止。
pub fn render_population_chart(
    mut gizmos: Gizmos,
    history: Res<PopulationHistory>,
    camera_query: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
) {
    let Ok((camera, camera_transform, projection)) = camera_query.get_single() else {
        return;
    };

    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };

    // 计算当前每个像素对应多少世界单位。
    let units_per_pixel = Vec2::new(
        projection.area.width() / viewport_size.x,
        projection.area.height() / viewport_size.y,
    );

    // 计算图表在屏幕上的像素尺寸对应的世界尺寸。
    let chart_size = Vec2::new(CHART_WIDTH_PX, CHART_HEIGHT_PX) * units_per_pixel;
    let margin = Vec2::splat(CHART_MARGIN_PX) * units_per_pixel;
    let gap = Vec2::new(0.0, CHART_GAP_PX) * units_per_pixel;

    // 屏幕右上角在相机本地坐标系中的世界偏移。
    // projection.area.max 是相机中心到右上角的偏移。
    let camera_pos = camera_transform.translation().xy();
    let screen_top_right = camera_pos
        + Vec2::new(
            projection.area.max.x - margin.x - chart_size.x,
            projection.area.max.y - margin.y - chart_size.y,
        );

    draw_mini_chart(
        &mut gizmos,
        screen_top_right,
        chart_size,
        "Creatures",
        Color::srgb(0.35, 0.65, 0.95),
        &history.creature_counts,
    );

    let bottom_left = screen_top_right - Vec2::new(0.0, chart_size.y + gap.y);
    draw_mini_chart(
        &mut gizmos,
        bottom_left,
        chart_size,
        "Food",
        Color::srgb(0.35, 0.85, 0.45),
        &history.food_counts,
    );
}

fn draw_mini_chart(
    gizmos: &mut Gizmos,
    bottom_left: Vec2,
    size: Vec2,
    _label: &str,
    color: Color,
    data: &VecDeque<f32>,
) {
    if data.len() < 2 {
        return;
    }

    let max_value = data.iter().copied().fold(0.0, f32::max).max(1.0);
    let data_len = data.len();

    // 绘制半透明背景框。
    let center = bottom_left + size * 0.5;
    gizmos.rect_2d(center, 0.0, size, Color::srgba(0.0, 0.0, 0.0, 0.35));

    // 绘制基线。
    gizmos.line_2d(
        bottom_left,
        bottom_left + Vec2::new(size.x, 0.0),
        Color::srgba(1.0, 1.0, 1.0, 0.25),
    );

    // 绘制数据线。
    let step_x = size.x / (MAX_SECONDS.saturating_sub(1).max(1) as f32);
    let mut prev: Option<Vec2> = None;

    for (i, &value) in data.iter().enumerate() {
        let x = bottom_left.x + step_x * ((MAX_SECONDS - data_len + i) as f32);
        let y = bottom_left.y + (value / max_value) * size.y;
        let point = Vec2::new(x, y);

        if let Some(p) = prev {
            gizmos.line_2d(p, point, color);
        }
        prev = Some(point);
    }
}
