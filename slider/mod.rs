use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod events;
pub mod systems;
pub mod builder;

pub use events::*;
pub use resources::*;
pub use systems::*;
pub use builder::SliderBuilder;

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SliderInputState>()
            .add_event::<SliderValueChangedEvent>()
            .configure_sets(
                Update,
                (
                    SliderSystem::ProcessInput
                        .after(bevy::input::InputSystem)
                        .before(bevy::ui::UiSystem::Layout),
                    SliderSystem::UpdateVisuals.after(SliderSystem::ProcessInput),
                )
                    .chain(),
            )
            .add_systems(Update, (
                slider_drag_system.in_set(SliderSystem::ProcessInput),
                slider_keyboard_input_system.in_set(SliderSystem::ProcessInput),
                slider_update_visuals_system.in_set(SliderSystem::UpdateVisuals),
            ));
    }
}
