use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct TabGroup {
    pub selected_tab: usize,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct TabGroupMeta {
    pub tab_names: Vec<String>,
    pub content_entities: Vec<Entity>,
    pub button_entities: Vec<Entity>,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct TabButton {
    pub tab_index: usize,
    pub group_entity: Entity,
}

#[derive(Component)]
pub struct TabActive;

#[derive(Component)]
pub struct TabNeedsVisualUpdate;

#[derive(Component)]
pub struct TabInactive;

#[derive(Component)]
pub struct TabDisabled;

#[derive(Component)]
pub struct TabFocused;

#[derive(Component)]
pub struct TabPanel;

#[derive(Component)]
pub struct TabHovered;

#[derive(Component)]
pub struct TabPressed;

#[derive(Component, Default)]
pub struct TabInteractionState {
    pub was_pressed: bool,
    pub was_hovered: bool,
}

#[derive(Component)]
pub struct TabContent {
    pub tab_index: usize,
    pub group_entity: Entity,
}

impl Default for TabContent {
    fn default() -> Self {
        Self {
            tab_index: 0,
            group_entity: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct TabGroupConfig {
    pub tab_style: TabStyle,
    pub panel_style: Node,
    pub animation_duration: f32,
    pub strategy: ContentStrategy,
    pub tab_button_style: Node,
    pub active_tab_style: Node,
    pub inactive_tab_style: Node,
    pub tab_spacing: f32,
}

impl Default for TabGroupConfig {
    fn default() -> Self {
        Self {
            tab_style: TabStyle::Top,
            panel_style: Node::default(),
            animation_duration: 0.2,
            strategy: ContentStrategy::Preloaded,
            tab_button_style: Node {
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::right(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            active_tab_style: Node::default(),
            inactive_tab_style: Node::default(),
            tab_spacing: 4.0,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect, Default)]
pub enum TabStyle {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Pill,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
pub enum ContentStrategy {
    #[default]
    Preloaded,
    LazyLoaded,
    Dynamic,
}

#[derive(Component, Default)]
pub struct TabAnimation {
    pub progress: f32,
    pub target: f32,
    pub velocity: f32,
}