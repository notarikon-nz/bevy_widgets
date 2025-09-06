use bevy::prelude::*;
use super::resources::DropdownOptionRegistry;
use bevy::ui::*;
use super::components::{ChildOf as DropdownChildOf, *};

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
        let config = self.config;
        
        let dropdown_entity = commands.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            BorderColor(Color::srgb(0.3, 0.3, 0.3)),
            BorderRadius::all(Val::Px(4.0)),
            Dropdown {
                option_ids: option_ids.clone(),
                selected_id: None,
                is_open: false,
                on_change: None,
            },
            config.clone(),
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
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            BorderColor(Color::srgb(0.6, 0.6, 0.6)),
            BorderRadius::all(Val::Px(4.0)),
            Text::new(&config.placeholder),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.2, 0.2, 0.2)),
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
        
        // Add option elements to the list
        let mut option_entities = Vec::new();
        for option_id in &option_ids {
            if let Some(option_data) = option_registry.options.get(option_id) {
                let option_entity = commands.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    Text::new(&option_data.label),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    DropdownOptionElement(*option_id),
                    DropdownChildOf::new(list_entity),
                )).id();
                option_entities.push(option_entity);
            }
        }
        
        commands.entity(list_entity).add_children(&option_entities);
        
        // Add ChildOf components for the main children
        commands.entity(button_entity).insert(DropdownChildOf::new(dropdown_entity));
        commands.entity(list_entity).insert(DropdownChildOf::new(dropdown_entity));
        commands.entity(backdrop_entity).insert(DropdownChildOf::new(dropdown_entity));
        
        commands.entity(dropdown_entity)
            .add_children(&[button_entity, list_entity, backdrop_entity]);
        
        dropdown_entity
    }
}