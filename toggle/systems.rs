use bevy::prelude::*;
use super::{components::*, events::*};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ToggleSystem {
    ProcessInput,
    UpdateAnimation,
    UpdateVisuals,
}

pub fn toggle_interaction_system(
    mut commands: Commands,
    mut q_toggles: Query<(Entity, &Interaction, &mut Toggle), (Changed<Interaction>, Without<ToggleDisabled>)>,
    mut evw_toggle_change: EventWriter<ToggleChangedEvent>,
) {
    for (entity, interaction, mut toggle) in &mut q_toggles {
        if let Interaction::Pressed = interaction {
            let previous_state = toggle.is_on;
            toggle.is_on = !toggle.is_on;
            
            evw_toggle_change.write(ToggleChangedEvent {
                toggle_entity: entity,
                previous_state,
                new_state: toggle.is_on,
                kind: ToggleChangeKind::User,
            });
            
            commands.entity(entity).insert(ToggleNeedsVisualUpdate);
        }
    }
}

pub fn toggle_keyboard_system(
    mut commands: Commands,
    mut q_toggles: Query<(Entity, &mut Toggle), With<ToggleFocused>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evw_toggle_change: EventWriter<ToggleChangedEvent>,
) {
    for (entity, mut toggle) in &mut q_toggles {
        if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
            let previous_state = toggle.is_on;
            toggle.is_on = !toggle.is_on;
            
            evw_toggle_change.write(ToggleChangedEvent {
                toggle_entity: entity,
                previous_state,
                new_state: toggle.is_on,
                kind: ToggleChangeKind::User,
            });
            
            commands.entity(entity).insert(ToggleNeedsVisualUpdate);
        }
    }
}

pub fn toggle_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut ToggleAnimation, &ToggleConfig, &Toggle)>,
) {
    for (mut animation, config, toggle) in &mut query {
        if !config.animated {
            animation.progress = if toggle.is_on { 1.0 } else { 0.0 };
            animation.target_progress = animation.progress;
            continue;
        }
        
        let target = if toggle.is_on { 1.0 } else { 0.0 };
        animation.target_progress = target;
        
        // Simple lerp for toggle (spring might be overkill)
        let speed = 1.0 / config.animation_duration.max(0.001);
        let delta = (animation.target_progress - animation.progress) * speed * time.delta_secs();
        animation.progress += delta;
        
        // Snap to target when close
        if (animation.target_progress - animation.progress).abs() < 0.01 {
            animation.progress = animation.target_progress;
        }
    }
}

pub fn toggle_visual_update_system(
    mut commands: Commands,
    mut q_toggles: Query<(
        Entity, 
        &Toggle, 
        &ToggleAnimation, 
        &ToggleConfig, 
        &ToggleParts,
        Option<&ToggleDisabled>
    ), With<ToggleNeedsVisualUpdate>>,
    mut q_tracks: Query<&mut BackgroundColor, With<ToggleTrack>>,
    mut q_knobs: Query<&mut Node, With<ToggleKnob>>,
) {
    for (entity, toggle, animation, config, parts, disabled) in &mut q_toggles {
        // Update track color
        if let Ok(mut track_color) = q_tracks.get_mut(parts.track) {
            let target_color = if toggle.is_on { config.on_color } else { config.off_color };
            if config.animated {
                *track_color = BackgroundColor(config.off_color.mix(
                    &config.on_color, 
                    animation.progress
                ));
            } else {
                *track_color = BackgroundColor(target_color);
            }
            
            if disabled.is_some() {
                track_color.0 = track_color.0.with_alpha(0.5);
            }
        }
        
        // Update knob position
        if let Ok(mut knob_style) = q_knobs.get_mut(parts.knob) {
            let knob_size = config.size.y - config.knob_margin * 2.0;
            let travel_distance = config.size.x - config.size.y;
            let x_position = config.knob_margin + (travel_distance * animation.progress);
            
            knob_style.left = Val::Px(x_position);
            knob_style.width = Val::Px(knob_size);
            knob_style.height = Val::Px(knob_size);
        }
        
        commands.entity(entity).remove::<ToggleNeedsVisualUpdate>();
    }
}

pub fn toggle_focus_system(
    mut commands: Commands,
    q_toggles: Query<(Entity, &Interaction), (With<Toggle>, Changed<Interaction>)>,
) {
    for (entity, interaction) in &q_toggles {
        match interaction {
            Interaction::Pressed => {
                commands.entity(entity).insert(ToggleFocused);
            }
            Interaction::None => {
                commands.entity(entity).remove::<ToggleFocused>();
            }
            _ => {}
        }
    }
}

#[cfg(debug_assertions)]
pub fn debug_toggle_lifecycle_system(
    mut removed_toggles: RemovedComponents<Toggle>,
    mut removed_state_scoped: RemovedComponents<StateScoped<crate::AppState>>,
) {
    for entity in removed_toggles.read() {
        info!("Toggle widget destroyed: entity {:?}", entity);
    }
    
    for entity in removed_state_scoped.read() {
        info!("State-scoped toggle widget destroyed due to state change: entity {:?}", entity);
    }
}