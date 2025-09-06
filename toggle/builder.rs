use bevy::prelude::*;
use super::components::*;

pub struct ToggleBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    initial_state: bool,
    config: ToggleConfig,
    disabled: bool,
}

impl<'w, 's, 'a> ToggleBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            initial_state: false,
            config: ToggleConfig::default(),
            disabled: false,
        }
    }

    pub fn with_initial_state(mut self, state: bool) -> Self {
        self.initial_state = state;
        self
    }

    pub fn with_config(mut self, config: ToggleConfig) -> Self {
        self.config = config;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn spawn(self) -> Entity {
        // Spawn track
        let track_entity = self.commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BorderRadius::all(Val::Px(12.0)),
            BackgroundColor(if self.initial_state {
                self.config.on_color
            } else {
                self.config.off_color
            }),
            ToggleTrack,
        )).id();

        // Spawn knob
        let knob_x = if self.initial_state {
            self.config.size.x - self.config.size.y + self.config.knob_margin
        } else {
            self.config.knob_margin
        };
        
        let knob_entity = self.commands.spawn((
            Node {
                width: Val::Px(self.config.size.y - self.config.knob_margin * 2.0),
                height: Val::Px(self.config.size.y - self.config.knob_margin * 2.0),
                position_type: PositionType::Absolute,
                left: Val::Px(knob_x),
                ..default()
            },
            BorderRadius::all(Val::Percent(50.0)),
            BackgroundColor(self.config.knob_color),
            ToggleKnob,
        )).id();

        let toggle_entity = self.commands.spawn((
            Button,
            Node {
                width: Val::Px(self.config.size.x),
                height: Val::Px(self.config.size.y),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            Toggle {
                is_on: self.initial_state,
            },
            self.config,
            ToggleAnimation {
                progress: if self.initial_state { 1.0 } else { 0.0 },
                target_progress: if self.initial_state { 1.0 } else { 0.0 },
                ..default()
            },
            ToggleParts {
                track: track_entity,
                knob: knob_entity,
            },
            ToggleNeedsVisualUpdate,
        )).id();

        if self.disabled {
            self.commands.entity(toggle_entity).insert(ToggleDisabled);
        }

        self.commands.entity(toggle_entity)
            .add_children(&[track_entity, knob_entity]);

        toggle_entity
    }

    pub fn spawn_state_scoped<T: States>(self, state: T) -> Entity {
        #[cfg(debug_assertions)]
        info!("Creating state-scoped toggle widget for state: {:?}", std::any::type_name::<T>());
        
        // Spawn all components in one go, including StateScoped
        // First spawn track
        let track_entity = self.commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BorderRadius::all(Val::Px(12.0)),
            BackgroundColor(if self.initial_state {
                self.config.on_color
            } else {
                self.config.off_color
            }),
            ToggleTrack,
        )).id();

        // Spawn knob
        let knob_x = if self.initial_state {
            self.config.size.x - self.config.size.y + self.config.knob_margin
        } else {
            self.config.knob_margin
        };
        
        let knob_entity = self.commands.spawn((
            Node {
                width: Val::Px(self.config.size.y - self.config.knob_margin * 2.0),
                height: Val::Px(self.config.size.y - self.config.knob_margin * 2.0),
                position_type: PositionType::Absolute,
                left: Val::Px(knob_x),
                ..default()
            },
            BorderRadius::all(Val::Percent(50.0)),
            BackgroundColor(self.config.knob_color),
            ToggleKnob,
        )).id();

        let toggle_entity = self.commands.spawn((
            Button,
            Node {
                width: Val::Px(self.config.size.x),
                height: Val::Px(self.config.size.y),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            Toggle {
                is_on: self.initial_state,
            },
            self.config,
            ToggleAnimation {
                progress: if self.initial_state { 1.0 } else { 0.0 },
                target_progress: if self.initial_state { 1.0 } else { 0.0 },
                ..default()
            },
            ToggleParts {
                track: track_entity,
                knob: knob_entity,
            },
            ToggleNeedsVisualUpdate,
            StateScoped(state), // Add StateScoped here
        )).id();

        if self.disabled {
            self.commands.entity(toggle_entity).insert(ToggleDisabled);
        }

        self.commands.entity(toggle_entity)
            .add_children(&[track_entity, knob_entity]);
            
        #[cfg(debug_assertions)]
        info!("State-scoped toggle widget created with entity: {:?}", toggle_entity);

        toggle_entity
    }
}