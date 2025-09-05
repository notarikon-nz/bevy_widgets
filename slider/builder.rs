use bevy::prelude::*;
use super::components::*;

pub struct SliderBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    bundle: SliderBundle,
    track_style: Style,
    handle_style: Style,
    fill_style: Style,
    text_style: TextStyle,
}

impl<'w, 's, 'a> SliderBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            bundle: SliderBundle::default(),
            track_style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            handle_style: Style {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            fill_style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            text_style: TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
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
            NodeBundle {
                style: self.track_style.clone(),
                background_color: BackgroundColor(Color::GRAY),
                ..default()
            },
            SliderTrack,
        )).id();

        let fill_entity = self.commands.spawn((
            NodeBundle {
                style: self.fill_style,
                background_color: BackgroundColor(Color::BLUE),
                ..default()
            },
            SliderFill,
        )).id();

        let handle_entity = self.commands.spawn((
            NodeBundle {
                style: self.handle_style,
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
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
            TextBundle {
                text: Text::from_section("", self.text_style),
                ..default()
            },
            SliderValueText,
        )).id();

        self.commands.entity(track_entity).push_children(&[fill_entity, handle_entity]);
        self.commands.entity(slider_entity).push_children(&[track_entity, text_entity]);

        slider_entity
    }
    
    pub fn spawn_with_parent(self, parent_entity: Entity) -> Entity {
        let slider_entity = self.spawn();
        self.commands.entity(parent_entity).push_children(&[slider_entity]);
        slider_entity
    }    
}