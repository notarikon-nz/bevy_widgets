use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub orientation: SliderOrientation,
    pub value: f32,
    pub handle_entity: Entity, // <- NEW: Track the handle entity
}

#[derive(Component, Debug, Clone)]
pub struct SliderOptions {
    pub format: ValueFormat,
    pub show_value: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum SliderEmitMode {
    Continuous,
    OnRelease,
}

#[derive(Component)]
pub struct SliderPendingChange {
    pub value: f32,
}

#[derive(Debug, Clone)]
pub enum ValueFormat {
    Precision(usize),
    Percent(usize),
    Custom(Arc<dyn Fn(f32) -> String + Send + Sync>),
}

impl Default for ValueFormat {
    fn default() -> Self {
        ValueFormat::Precision(2)
    }
}

impl ValueFormat {
    pub fn format(&self, value: f32) -> String {
        match self {
            ValueFormat::Precision(p) => format!("{:.*}", p, value),
            ValueFormat::Percent(p) => format!("{:.*}%", p, value * 100.0),
            ValueFormat::Custom(f) => f(value),
        }
    }
}

#[derive(Component)]
pub struct SliderHandle;

#[derive(Component)]
pub struct SliderTrack;

#[derive(Component)]
pub struct SliderFill;

#[derive(Component)]
pub struct SliderValueText;

#[derive(Component)]
pub struct SliderNeedsVisualUpdate;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

#[derive(Bundle)]
pub struct SliderBundle {
    pub slider: Slider,
    pub options: SliderOptions,
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
}

impl Default for SliderBundle {
    fn default() -> Self {
        Self {
            slider: Slider {
                min: 0.0,
                max: 1.0,
                step: None,
                orientation: SliderOrientation::Horizontal,
                value: 0.5,
                handle_entity: Entity::PLACEHOLDER,
            },
            options: SliderOptions {
                format: ValueFormat::default(),
                show_value: true,
            },
            node: Node::default(),
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(24.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            focus_policy: FocusPolicy::Block,
        }
    }
}
