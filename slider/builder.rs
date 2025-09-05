use bevy::prelude::*;
use bevy::ui::*;
use super::components::*;
use std::sync::Arc;

pub struct SliderBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    bundle: SliderBundle,
    track_node: Node,
    handle_node: Node,
    fill_node: Node,
    text_style: TextFont,
}

impl<'w, 's, 'a> SliderBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            bundle: SliderBundle::default(),
            track_node: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            handle_node: Node {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            fill_node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            text_style: TextFont {
                font_size: 16.0,
                ..default()
            },
        }
    }

    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.bundle.slider.min = min;
        self.bundle.slider.max = max;
        self.bundle.slider.value = min.clamp(min, max);
        self
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.bundle.slider.value = value.clamp(self.bundle.slider.min, self.bundle.slider.max);
        self
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.bundle.slider.step = Some(step);
        self
    }

    pub fn with_orientation(mut self, orientation: SliderOrientation) -> Self {
        self.bundle.slider.orientation = orientation;
        self
    }

    pub fn with_value_formatter<F>(mut self, formatter: F) -> Self
    where
        F: Fn(f32) -> String + Send + Sync + 'static,
    {
        self.bundle.options.format = ValueFormat::Custom(Arc::new(formatter));
        self
    }

    pub fn spawn(self) -> Entity {
        // let slider_entity = self.commands.spawn(self.bundle).id();

        let track_entity = self.commands.spawn((
            self.track_node,
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            SliderTrack,
        )).id();

        let fill_entity = self.commands.spawn((
            self.fill_node,
            BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
            SliderFill,
        )).id();

        let handle_entity = self.commands.spawn((
            self.handle_node,
            BackgroundColor(Color::WHITE),
            SliderHandle,
            Interaction::None,
            FocusPolicy::Pass,
        )).id();
        
	    let slider_entity = self.commands.spawn(SliderBundle {
	        slider: Slider {
	            handle_entity, // <- Store the handle entity
	            ..self.bundle.slider
	        },
	        ..self.bundle
	    }).id();        

        let text_entity = self.commands.spawn((
            Text::new(""),
            self.text_style,
            TextColor(Color::WHITE),
            SliderValueText,
        )).id();

        self.commands.entity(track_entity).add_children(&[fill_entity, handle_entity]);
        self.commands.entity(slider_entity).add_children(&[track_entity, text_entity]);

        slider_entity
    }
    
    pub fn spawn_with_parent(self, parent_entity: Entity) -> Entity {
        // Duplicate the spawn logic to avoid the move issue
        let track_entity = self.commands.spawn((
            self.track_node,
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            SliderTrack,
        )).id();

        let fill_entity = self.commands.spawn((
            self.fill_node,
            BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
            SliderFill,
        )).id();

        let handle_entity = self.commands.spawn((
            self.handle_node,
            BackgroundColor(Color::WHITE),
            SliderHandle,
            Interaction::None,
            FocusPolicy::Pass,
        )).id();
        
        let slider_entity = self.commands.spawn(SliderBundle {
            slider: Slider {
                handle_entity,
                ..self.bundle.slider
            },
            ..self.bundle
        }).id();        

        let text_entity = self.commands.spawn((
            Text::new(""),
            self.text_style,
            TextColor(Color::WHITE),
            SliderValueText,
        )).id();

        self.commands.entity(track_entity).add_children(&[fill_entity, handle_entity]);
        self.commands.entity(slider_entity).add_children(&[track_entity, text_entity]);
        self.commands.entity(parent_entity).add_children(&[slider_entity]);

        slider_entity
    }    
}