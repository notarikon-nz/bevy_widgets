use bevy::prelude::*;
use super::resources::DropdownOptionRegistry;
use bevy::ui::*;
use super::components::*;
use std::sync::Arc;

pub struct DropdownBuilder {
    options: Vec<(String, Option<Handle<Image>>)>,
    config: DropdownConfig,
}

impl DropdownBuilder {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            config: DropdownConfig::default(),
        }
    }
    
    pub fn with_option(mut self, label: impl Into<String>, icon: Option<Handle<Image>>) -> Self {
        self.options.push((label.into(), icon));
        self
    }
    
    pub fn with_config(mut self, config: DropdownConfig) -> Self {
        self.config = config;
        self
    }
    
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.config.placeholder = placeholder.into();
        self
    }
    
    pub fn build(self) -> DropdownSpawnCommand {
        DropdownSpawnCommand {
            options: self.options,
            config: self.config,
        }
    }
}

pub struct DropdownSpawnCommand {
    options: Vec<(String, Option<Handle<Image>>)>,
    config: DropdownConfig,
}

impl DropdownSpawnCommand {
    pub fn spawn(
        self,
        commands: &mut Commands,
        option_registry: &mut DropdownOptionRegistry,
    ) -> Entity {
        let option_ids = option_registry.register_options(self.options);
        
        let dropdown_entity = commands.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                ..default()
            },
            Dropdown {
                option_ids,
                selected_id: None,
                is_open: false,
                on_change: None,
            },
            self.config,
            DropdownAnimation::default(),
            Interaction::None,
            FocusPolicy::Block,
            DropdownNeedsVisualUpdate,
        )).id();
        
        // Spawn button
        let button_entity = commands.spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            DropdownButton,
        )).id();
        
        // Spawn list (initially hidden)
        let list_entity = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(0.0),
                position_type: PositionType::Absolute,
                top: Val::Px(40.0),
                overflow: Overflow::clip(),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Visibility::Hidden,
            DropdownList,
        )).id();
        
        // Spawn backdrop (initially hidden)
        let backdrop_entity = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::NONE),
            Visibility::Hidden,
            DropdownBackdrop,
        )).id();
        
        commands.entity(dropdown_entity)
            .add_children(&[button_entity, list_entity, backdrop_entity]);
        
        dropdown_entity
    }
}