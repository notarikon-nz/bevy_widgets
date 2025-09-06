use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Toggle {
    pub is_on: bool,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct ToggleParts {
    pub track: Entity,
    pub knob: Entity,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct ToggleConfig {
    pub animated: bool,
    pub animation_duration: f32,
    pub on_color: Color,
    pub off_color: Color,
    pub knob_color: Color,
    pub size: Vec2,
    pub knob_margin: f32,
    pub drag_threshold: f32,
}

impl Default for ToggleConfig {
    fn default() -> Self {
        Self {
            animated: true,
            animation_duration: 0.2,
            on_color: Color::srgb(0.2, 0.8, 0.2),
            off_color: Color::srgb(0.5, 0.5, 0.5),
            knob_color: Color::WHITE,
            size: Vec2::new(50.0, 25.0),
            knob_margin: 2.0,
            drag_threshold: 5.0,
        }
    }
}

#[derive(Component)]
pub struct ToggleTrack;

#[derive(Component)]
pub struct ToggleKnob;

#[derive(Component)]
pub struct ToggleNeedsVisualUpdate;

#[derive(Component)]
pub struct ToggleFocused;

#[derive(Component)]
pub struct ToggleDisabled;

#[derive(Component, Default)]
pub struct ToggleAnimation {
    pub progress: f32,
    pub target_progress: f32,
    pub velocity: f32,
}

#[derive(Component, Default)]
pub struct ToggleDragState {
    pub is_dragging: bool,
    pub drag_start_position: Vec2,
    pub drag_start_progress: f32,
}