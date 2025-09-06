use bevy::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct TabChangedEvent {
    pub group_entity: Entity,
    pub previous_tab: usize,
    pub new_tab: usize,
    pub change_kind: TabChangeKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabChangeKind {
    UserInteraction,
    Programmatic,
    Initialization,
}