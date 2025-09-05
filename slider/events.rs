use bevy::prelude::*;
use super::components::SliderOrientation;

#[derive(Event, Debug, Clone)]
pub struct SliderValueChangedEvent {
    pub entity: Entity,
    pub handle_entity: Entity,
    pub previous_value: f32,
    pub new_value: f32,
    pub orientation: SliderOrientation,
}

