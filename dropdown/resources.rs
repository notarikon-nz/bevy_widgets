use bevy::prelude::*;
use super::components::DropdownOptionId;

#[derive(Resource, Default)]
pub struct DropdownOptionRegistry {
    pub options: std::collections::HashMap<DropdownOptionId, DropdownOption>,
    next_id: DropdownOptionId,
}

#[derive(Debug, Clone)]
pub struct DropdownOption {
    pub label: String,
    pub icon: Option<Handle<Image>>,
}

impl DropdownOptionRegistry {
    pub fn register_option(&mut self, label: String, icon: Option<Handle<Image>>) -> DropdownOptionId {
        let id = self.next_id;
        self.next_id += 1;
        self.options.insert(id, DropdownOption { label, icon });
        id
    }
    
    pub fn register_options(&mut self, options: Vec<(String, Option<Handle<Image>>)>) -> Vec<DropdownOptionId> {
        options.into_iter()
            .map(|(label, icon)| self.register_option(label, icon))
            .collect()
    }
}

#[derive(Resource)]
pub struct UiZIndexAllocator {
    next_z_index: i32,
}

impl Default for UiZIndexAllocator {
    fn default() -> Self {
        Self { next_z_index: 1 }
    }
}

impl UiZIndexAllocator {
    pub fn next(&mut self) -> i32 {
        let result = self.next_z_index;
        self.next_z_index = self.next_z_index.wrapping_add(1);
        
        if self.next_z_index < 1 || self.next_z_index > 1_000_000 {
            self.next_z_index = 1;
        }
        
        result
    }
}