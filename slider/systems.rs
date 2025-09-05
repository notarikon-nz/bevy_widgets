use bevy::prelude::*;
use super::{components::*, events::*, resources::SliderInputState};
use std::time::Duration;
use std::sync::Arc;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SliderSystem {
    ProcessInput,
    UpdateVisuals,
}

pub fn slider_drag_system(
    mut commands: Commands,
    mut q_handles: Query<
        (
            Entity,
            &Interaction,
            &Parent,
            &GlobalTransform,
            &Style,
        ),
        (With<SliderHandle>, Changed<Interaction>)
    >,
    q_sliders: Query<(&Slider, &SliderOptions)>,
    q_tracks: Query<(&GlobalTransform, &Style), With<SliderTrack>>,
    q_nodes: Query<&Node>, // <- ADDED: Query for computed node sizes
    mut evr_cursor: EventReader<CursorMoved>,
    mut evw_slider_change: EventWriter<SliderValueChangedEvent>,
) {
    let cursor_position = evr_cursor.read().last().map(|ev| ev.position);

    for (handle_entity, interaction, parent, handle_transform, handle_style) in &mut q_handles {
        if let (Interaction::Pressed, Some(cursor_pos)) = (interaction, cursor_position) {
            if let Ok(track_node) = q_nodes.get(parent.get()) {
                if let Ok((track_transform, track_style)) = q_tracks.get(parent.get()) {
                    if let Ok((slider, options)) = q_sliders.get(parent.get()) {
                        let new_value = cursor_position_to_value(
                            cursor_pos,
                            track_transform,
                            track_node.size(),
                            slider,
                        );

                    let stepped_new_value = apply_step(new_value, slider.min, slider.step);
                    let clamped_new_value = stepped_new_value.clamp(slider.min, slider.max);

                    if (clamped_new_value - slider.value).abs() > f32::EPSILON {
                        let previous_value = slider.value;
                        
                        commands.entity(parent.get()).insert(SliderNeedsVisualUpdate);
                        
                            evw_slider_change.send(SliderValueChangedEvent {
                                entity: parent.get(),
                                handle_entity,
                                previous_value,
                                new_value: clamped_new_value,
                                orientation: slider.orientation,
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn slider_keyboard_input_system(
    time: Res<Time>,
    mut input_state: ResMut<SliderInputState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q_sliders: Query<(Entity, &mut Slider, &SliderOptions)>,
    mut evw_slider_change: EventWriter<SliderValueChangedEvent>,
    mut commands: Commands,
) {
    let mut step_direction = 0.0;

    for &key in [KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowDown, KeyCode::ArrowUp].iter() {
        if keys.just_pressed(key) {
            input_state.held_key = Some(key);
            input_state.key_repeat_timer.reset();
            step_direction = match key {
                KeyCode::ArrowRight | KeyCode::ArrowUp => 1.0,
                KeyCode::ArrowLeft | KeyCode::ArrowDown => -1.0,
                _ => 0.0,
            };
            break;
        }
    }

    if let Some(held_key) = input_state.held_key {
        if keys.pressed(held_key) {
            input_state.key_repeat_timer.tick(time.delta());
            if input_state.key_repeat_timer.just_finished() {
                step_direction = match held_key {
                    KeyCode::ArrowRight | KeyCode::ArrowUp => 1.0,
                    KeyCode::ArrowLeft | KeyCode::ArrowDown => -1.0,
                    _ => 0.0,
                };
                input_state.key_repeat_timer = Timer::new(Duration::from_millis(50), TimerMode::Once);
                input_state.key_repeat_timer.reset();
            }
        } else {
            input_state.held_key = None;
        }
    }

    if step_direction != 0.0 {
        for (entity, mut slider, options) in &mut q_sliders {
            let step_amount = slider.step.unwrap_or((slider.max - slider.min) * 0.05);
            let new_value = apply_step(
                slider.value + (step_direction * step_amount),
                slider.min,
                slider.step,
            ).clamp(slider.min, slider.max);

            if (new_value - slider.value).abs() > f32::EPSILON {
                let previous_value = slider.value;
                slider.value = new_value;

                evw_slider_change.send(SliderValueChangedEvent {
                    entity,
                    handle_entity: slider.handle_entity, // Would need to be looked up
                    previous_value,
                    new_value,
                    orientation: slider.orientation,
                });
                
                commands.entity(entity).insert(SliderNeedsVisualUpdate);
            }
        }
    }
}

pub fn slider_update_visuals_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_sliders: Query<
        (Entity, &Slider, &SliderOptions, &Children),
        With<SliderNeedsVisualUpdate>
    >,
    mut q_handles: Query<&mut Style, With<SliderHandle>>,
    mut q_fills: Query<&mut Style, With<SliderFill>>,
    mut q_text: Query<&mut Text, With<SliderValueText>>,
) {
    for (slider_entity, slider, options, children) in &mut q_sliders {
        let normalized_value = (slider.value - slider.min) / (slider.max - slider.min);
        
        for &child in children.iter() {
            if let Ok(mut handle_style) = q_handles.get_mut(child) {
                match slider.orientation {
                    SliderOrientation::Horizontal => {
                        handle_style.left = Val::Percent(normalized_value * 100.0);
                    }
                    SliderOrientation::Vertical => {
                        handle_style.bottom = Val::Percent(normalized_value * 100.0);
                    }
                }
            }
            
            if let Ok(mut fill_style) = q_fills.get_mut(child) {
                match slider.orientation {
                    SliderOrientation::Horizontal => {
                        fill_style.width = Val::Percent(normalized_value * 100.0);
                    }
                    SliderOrientation::Vertical => {
                        fill_style.height = Val::Percent(normalized_value * 100.0);
                    }
                }
            }
            
            if let Ok(mut text) = q_text.get_mut(child) {
                text.sections[0].value = options.format.format(slider.value);
            }
        }
        
        commands.entity(slider_entity).remove::<SliderNeedsVisualUpdate>();
    }
}

fn apply_step(value: f32, min: f32, step: Option<f32>) -> f32 {
    let Some(step) = step else {
        return value;
    };
    let steps = ((value - min) / step).round();
    min + (steps * step)
}

fn cursor_position_to_value(
    cursor_pos: Vec2,
    track_global_transform: &GlobalTransform,
    track_size: Vec2, // <- Now takes computed Vec2 size
    slider: &Slider,
) -> f32 {
    let track_center = track_global_transform.translation();
    let relative_pos = match slider.orientation {
        SliderOrientation::Horizontal => {
            (cursor_pos.x - (track_center.x - track_size.x / 2.0)) / track_size.x
        }
        SliderOrientation::Vertical => {
            (cursor_pos.y - (track_center.y - track_size.y / 2.0)) / track_size.y
        }
    };
    slider.min + (relative_pos.clamp(0.0, 1.0) * (slider.max - slider.min))
}

pub fn slider_buffer_changes_system(
    mut commands: Commands,
    mut evr_slider_change: EventReader<SliderValueChangedEvent>,
    q_sliders: Query<&SliderEmitMode>,
) {
    for event in evr_slider_change.read() {
        if let Ok(emit_mode) = q_sliders.get(event.entity) {
            if matches!(emit_mode, SliderEmitMode::OnRelease) {
                commands.entity(event.entity).insert(SliderPendingChange {
                    value: event.new_value,
                });
                // Don't send the event downstream yet
            }
        }
    }
}

pub fn slider_emit_buffered_changes_system(
    mut commands: Commands,
    mut q_sliders: Query<(Entity, &SliderPendingChange, &Interaction), Changed<Interaction>>,
    mut evw_final_change: EventWriter<SliderValueChangedEvent>,
) {
    for (entity, pending_change, interaction) in &mut q_sliders {
        if matches!(interaction, Interaction::None) {
            // Note: This would need the full event data from the original slider
            commands.entity(entity).remove::<SliderPendingChange>();
        }
    }
}