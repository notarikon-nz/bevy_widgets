use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;
pub mod builder;

pub use components::*;
pub use events::*;
pub use systems::*;
pub use builder::*;

pub struct TogglePlugin;

impl Plugin for TogglePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ToggleChangedEvent>()
            .configure_sets(
                Update,
                (
                    ToggleSystem::ProcessInput
                        .after(bevy::input::InputSystem)
                        .before(bevy::ui::UiSystem::Layout),
                    ToggleSystem::UpdateAnimation.after(ToggleSystem::ProcessInput),
                    ToggleSystem::UpdateVisuals.after(ToggleSystem::UpdateAnimation),
                )
                .chain(),
            )
            .add_systems(Update, (
                toggle_interaction_system.in_set(ToggleSystem::ProcessInput),
                toggle_keyboard_system.in_set(ToggleSystem::ProcessInput),
                toggle_focus_system.in_set(ToggleSystem::ProcessInput),
                toggle_animation_system.in_set(ToggleSystem::UpdateAnimation),
                toggle_visual_update_system.in_set(ToggleSystem::UpdateVisuals),
            ));
        
        #[cfg(debug_assertions)]
        app.add_systems(Update, debug_toggle_lifecycle_system);
    }
}