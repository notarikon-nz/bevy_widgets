use bevy::prelude::*;
use super::components::DropdownOptionId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DropdownChangeKind {
    Opened,
    Closed,
    SelectionChanged,
    Cancelled,
}

#[derive(Event, Debug, Clone)]
pub struct DropdownChangedEvent {
    pub dropdown_entity: Entity,
    pub kind: DropdownChangeKind,
    pub previous_id: Option<DropdownOptionId>,
    pub new_id: Option<DropdownOptionId>,
    pub previous_label: Option<String>,
    pub new_label: Option<String>,
}