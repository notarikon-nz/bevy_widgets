use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToggleChangeKind {
    User,
    Programmatic,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ToggleChangedEvent {
    pub toggle_entity: Entity,
    pub previous_state: bool,
    pub new_state: bool,
    pub kind: ToggleChangeKind,
}