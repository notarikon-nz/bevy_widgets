use bevy::prelude::*;
use super::{components::{ChildOf as DropdownChildOf, *}, events::*, resources::*};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum DropdownSystem {
    ProcessInput,
    UpdateAnimation,
    UpdateVisuals,
}

pub fn dropdown_toggle_system(
    mut commands: Commands,
    mut q_dropdowns: Query<(Entity, &mut Dropdown, &Children)>,
    q_buttons: Query<&Interaction, (With<DropdownButton>, Changed<Interaction>)>,
    mut evw_dropdown_change: EventWriter<DropdownChangedEvent>,
) {
    for (dropdown_entity, mut dropdown, children) in &mut q_dropdowns {
        for child in children.iter() {
            if let Ok(interaction) = q_buttons.get(child) {
                if let Interaction::Pressed = interaction {
                    dropdown.is_open = !dropdown.is_open;
                    
                    let kind = if dropdown.is_open {
                        DropdownChangeKind::Opened
                    } else {
                        DropdownChangeKind::Closed
                    };
                    
                    evw_dropdown_change.write(DropdownChangedEvent {
                        dropdown_entity,
                        kind,
                        previous_id: dropdown.selected_id,
                        new_id: dropdown.selected_id,
                        previous_label: None,
                        new_label: None,
                    });
                    
                    commands.entity(dropdown_entity).insert(DropdownNeedsVisualUpdate);
                    
                    if dropdown.is_open {
                        commands.entity(dropdown_entity).insert(DropdownFocused);
                    } else {
                        commands.entity(dropdown_entity).remove::<DropdownFocused>();
                    }
                }
            }
        }
    }
}

pub fn dropdown_backdrop_system(
    mut commands: Commands,
    mut q_backdrops: Query<(Entity, &Interaction), (With<DropdownBackdrop>, Changed<Interaction>)>,
    q_parents: Query<&DropdownChildOf, With<DropdownBackdrop>>,
    mut q_dropdowns: Query<(Entity, &mut Dropdown, &Children)>,
    mut evw_dropdown_change: EventWriter<DropdownChangedEvent>,
) {
    for (backdrop_entity, interaction) in &mut q_backdrops {
        if let Interaction::Pressed = interaction {
            if let Ok(parent) = q_parents.get(backdrop_entity) {
                if let Ok((dropdown_entity, mut dropdown, _)) = q_dropdowns.get_mut(parent.parent()) {
                    dropdown.is_open = false;
                    
                    evw_dropdown_change.write(DropdownChangedEvent {
                        dropdown_entity,
                        kind: DropdownChangeKind::Cancelled,
                        previous_id: dropdown.selected_id,
                        new_id: dropdown.selected_id,
                        previous_label: None,
                        new_label: None,
                    });
                    
                    commands.entity(dropdown_entity)
                        .insert(DropdownNeedsVisualUpdate)
                        .remove::<DropdownFocused>();
                }
            }
        }
    }
}

pub fn dropdown_option_select_system(
    mut commands: Commands,
    option_registry: Res<DropdownOptionRegistry>,
    mut q_dropdowns: Query<(Entity, &mut Dropdown)>,
    q_option_parents: Query<&DropdownChildOf, With<DropdownOptionElement>>,
    mut q_options: Query<(Entity, &Interaction, &DropdownOptionElement), Changed<Interaction>>,
    mut evw_dropdown_change: EventWriter<DropdownChangedEvent>,
) {
    for (option_entity, interaction, option_element) in &mut q_options {
        if let Interaction::Pressed = interaction {
            if let Ok(parent) = q_option_parents.get(option_entity) {
                // The parent is the list entity, we need the grandparent (dropdown entity)
                if let Ok(list_parent) = q_option_parents.get(parent.parent()) {
                    if let Ok((dropdown_entity, mut dropdown)) = q_dropdowns.get_mut(list_parent.parent()) {
                        let previous_id = dropdown.selected_id;
                        let previous_label = previous_id.and_then(|id| 
                            option_registry.options.get(&id).map(|o| o.label.clone())
                        );
                        
                        dropdown.selected_id = Some(option_element.0);
                        dropdown.is_open = false;
                        
                        let new_label = option_registry.options.get(&option_element.0)
                            .map(|o| o.label.clone());
                        
                        evw_dropdown_change.write(DropdownChangedEvent {
                            dropdown_entity,
                            kind: DropdownChangeKind::SelectionChanged,
                            previous_id,
                            new_id: Some(option_element.0),
                            previous_label,
                            new_label,
                        });
                        
                        commands.entity(dropdown_entity)
                            .insert(DropdownNeedsVisualUpdate)
                            .remove::<DropdownFocused>();
                        
                        if let Some(callback) = &dropdown.on_change {
                            callback(Some(option_element.0));
                        }
                    }
                }
            }
        }
    }
}

pub fn dropdown_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut DropdownAnimation, &DropdownConfig, &Dropdown)>,
) {
    for (mut animation, config, dropdown) in &mut query {
        let target = if dropdown.is_open { 1.0 } else { 0.0 };
        animation.target_progress = target;
        
        let displacement = animation.target_progress - animation.progress;
        let spring_force = config.animation_config.stiffness * displacement;
        let damping_force = config.animation_config.damping * animation.velocity;
        let acceleration = spring_force - damping_force;
        
        animation.velocity += acceleration * time.delta_secs();
        animation.progress += animation.velocity * time.delta_secs();
        
        if displacement.abs() < config.animation_config.precision 
            && animation.velocity.abs() < config.animation_config.precision {
            animation.progress = animation.target_progress;
            animation.velocity = 0.0;
        }
    }
}

pub fn dropdown_visual_update_system(
    mut commands: Commands,
    option_registry: Res<DropdownOptionRegistry>,
    mut q_dropdowns: Query<(Entity, &Dropdown, &DropdownAnimation, &DropdownConfig, &Children), With<DropdownNeedsVisualUpdate>>,
    mut q_buttons: Query<&mut Text, With<DropdownButton>>,
    mut q_lists: Query<(&mut Node, &mut Visibility, &mut Transform), With<DropdownList>>,
) {
    for (entity, dropdown, animation, config, children) in &mut q_dropdowns {
        for child in children.iter() {
            if let Ok(mut text) = q_buttons.get_mut(child) {
                let display_text = if let Some(id) = dropdown.selected_id {
                    option_registry.options.get(&id).map(|o| o.label.clone())
                        .unwrap_or_else(|| "Invalid option".to_string())
                } else {
                    config.placeholder.clone()
                };
                **text = display_text;
            }
            
            if let Ok((mut list_style, mut visibility, mut transform)) = q_lists.get_mut(child) {
                let max_height = match config.max_height {
                    Val::Px(px) => px,
                    _ => 200.0,
                };
                let height = animation.progress * max_height;
                
                if animation.progress <= config.animation_config.precision {
                    *visibility = Visibility::Hidden;
                } else {
                    *visibility = Visibility::Visible;
                }
                
                list_style.height = Val::Px(height);
                
                match config.direction {
                    DropdownDirection::Down => {
                        transform.translation.y = 0.0;
                    }
                    DropdownDirection::Up => {
                        transform.translation.y = -height * (1.0 - animation.progress);
                    }
                    DropdownDirection::Auto => {
                        transform.translation.y = 0.0;
                    }
                }
            }
        }
        
        commands.entity(entity).remove::<DropdownNeedsVisualUpdate>();
    }
}

pub fn dropdown_z_index_system(
    mut allocator: ResMut<UiZIndexAllocator>,
    mut q_dropdowns: Query<(&mut ZIndex, &DropdownAnimation), With<DropdownList>>,
) {
    for (mut z_index, animation) in &mut q_dropdowns {
        if animation.progress > 0.0 && z_index.0 == 0 {
            z_index.0 = allocator.next();
        } else if animation.progress <= 0.01 && z_index.0 > 0 {
            z_index.0 = 0;
        }
    }
}

pub fn dropdown_focus_management_system(
    mut commands: Commands,
    q_dropdowns: Query<(Entity, &DropdownAnimation, &DropdownConfig)>,
    q_focused: Query<Entity, With<DropdownFocused>>,
) {
    for (entity, animation, config) in &q_dropdowns {
        if animation.progress <= config.animation_config.precision {
            if let Ok(focused_entity) = q_focused.get(entity) {
                commands.entity(focused_entity).remove::<DropdownFocused>();
            }
        }
    }
}

pub fn dropdown_keyboard_system(
    mut q_dropdowns: Query<(Entity, &mut Dropdown, &DropdownAnimation), With<DropdownFocused>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evw_dropdown_change: EventWriter<DropdownChangedEvent>,
    mut commands: Commands,
) {
    for (entity, mut dropdown, animation) in &mut q_dropdowns {
        if keys.just_pressed(KeyCode::Escape) && dropdown.is_open {
            dropdown.is_open = false;
            
            evw_dropdown_change.write(DropdownChangedEvent {
                dropdown_entity: entity,
                kind: DropdownChangeKind::Cancelled,
                previous_id: dropdown.selected_id,
                new_id: dropdown.selected_id,
                previous_label: None,
                new_label: None,
            });
            
            commands.entity(entity).insert(DropdownNeedsVisualUpdate);
        }
    }
}