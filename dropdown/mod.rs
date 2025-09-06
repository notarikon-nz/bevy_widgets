pub mod components;
pub mod resources;
pub mod events;
pub mod systems;
pub mod builder;
pub use components::*;
pub use events::*;
pub use resources::*;
pub use builder::{DropdownBuilder};

use bevy::prelude::*;
use systems::*;

pub struct DropdownPlugin;

impl Plugin for DropdownPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DropdownOptionRegistry>()
            .init_resource::<UiZIndexAllocator>()
            .add_event::<DropdownChangedEvent>()
            .configure_sets(
                Update,
                (
                    DropdownSystem::ProcessInput
                        .after(bevy::input::InputSystem)
                        .before(bevy::ui::UiSystem::Layout),
                    DropdownSystem::UpdateAnimation.after(DropdownSystem::ProcessInput),
                    DropdownSystem::UpdateVisuals.after(DropdownSystem::UpdateAnimation),
                )
                .chain(),
            )
            .add_systems(Update, (
                dropdown_toggle_system.in_set(DropdownSystem::ProcessInput),
                dropdown_backdrop_system.in_set(DropdownSystem::ProcessInput),
                dropdown_option_select_system.in_set(DropdownSystem::ProcessInput),
                dropdown_keyboard_system.in_set(DropdownSystem::ProcessInput),
                dropdown_animation_system.in_set(DropdownSystem::UpdateAnimation),
                dropdown_visual_update_system.in_set(DropdownSystem::UpdateVisuals),
                dropdown_z_index_system,
                dropdown_focus_management_system,
            ));
    }
}