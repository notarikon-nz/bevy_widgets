use bevy::prelude::*;
use super::{events::*, systems::*};

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TabChangedEvent>()
            .configure_sets(
                Update,
                (
                    TabSystem::ProcessInput
                        .after(bevy::input::InputSystem)
                        .before(bevy::ui::UiSystem::Layout),
                    TabSystem::UpdateContent.after(TabSystem::ProcessInput),
                )
                .chain(),
            )
            .add_systems(Update, (
                tab_button_interaction_system.in_set(TabSystem::ProcessInput),
                tab_keyboard_navigation_system.in_set(TabSystem::ProcessInput),
                tab_focus_system.in_set(TabSystem::ProcessInput),
                tab_content_management_system.in_set(TabSystem::UpdateContent),
                tab_content_visibility_system.after(TabSystem::UpdateContent), // Responds to events
            ))
            .add_systems(Update, tab_continuous_visual_update_system); // Independent continuous system
    }
}