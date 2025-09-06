use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Dropdown {
    pub option_ids: Vec<DropdownOptionId>,
    pub selected_id: Option<DropdownOptionId>,
    pub is_open: bool,
    #[reflect(ignore)]
    pub on_change: Option<Box<dyn Fn(Option<DropdownOptionId>) + Send + Sync>>,
}

impl Clone for Dropdown {
    fn clone(&self) -> Self {
        Self {
            option_ids: self.option_ids.clone(),
            selected_id: self.selected_id,
            is_open: self.is_open,
            on_change: None, // Don't clone function pointers
        }
    }
}

impl std::fmt::Debug for Dropdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dropdown")
            .field("option_ids", &self.option_ids)
            .field("selected_id", &self.selected_id)
            .field("is_open", &self.is_open)
            .field("on_change", &self.on_change.as_ref().map(|_| "Some(fn)"))
            .finish()
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct DropdownConfig {
    pub max_height: Val,
    pub direction: DropdownDirection,
    pub searchable: bool,
    pub placeholder: String,
    pub animation_config: AnimationConfig,
}

impl Default for DropdownConfig {
    fn default() -> Self {
        Self {
            max_height: Val::Px(200.0),
            direction: DropdownDirection::Auto,
            searchable: false,
            placeholder: "Select an option...".to_string(),
            animation_config: AnimationConfig {
                stiffness: 170.0,
                damping: 26.0,
                precision: 0.01,
            },
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub enum DropdownDirection {
    Down,
    Up,
    Auto,
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct AnimationConfig {
    pub stiffness: f32,
    pub damping: f32,
    pub precision: f32,
}

#[derive(Component)]
pub struct DropdownButton;

#[derive(Component)]
pub struct DropdownList;

#[derive(Component)]
pub struct DropdownOptionElement(pub DropdownOptionId);

#[derive(Component)]
pub struct DropdownBackdrop;

#[derive(Component)]
pub struct DropdownFocused;

#[derive(Component)]
pub struct DropdownNeedsVisualUpdate;

#[derive(Component, Default)]
pub struct DropdownAnimation {
    pub progress: f32,
    pub target_progress: f32,
    pub velocity: f32,
}

pub type DropdownOptionId = u32;

#[derive(Component)]
pub struct ChildOf {
    parent: Entity,
}

impl ChildOf {
    pub fn new(parent: Entity) -> Self {
        Self { parent }
    }
    
    pub fn parent(&self) -> Entity {
        self.parent
    }
}