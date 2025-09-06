use bevy::prelude::*;
use super::{components::*, events::*};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TabSystem {
    ProcessInput,
    UpdateContent,
    UpdateVisuals,
}

pub fn tab_button_interaction_system(
    mut commands: Commands,
    q_tab_buttons: Query<(Entity, &Interaction, &TabButton), Changed<Interaction>>,
    mut q_tab_groups: Query<(&mut TabGroup, &TabGroupMeta)>,
    mut evw_tab_change: EventWriter<TabChangedEvent>,
) {
    for (button_entity, interaction, tab_button) in q_tab_buttons.iter() {
        #[cfg(debug_assertions)]
        info!("Tab button {:?} interaction: {:?}, tab_index: {}, group_entity: {:?}", 
              button_entity, interaction, tab_button.tab_index, tab_button.group_entity);
        
        match interaction {
            Interaction::Pressed => {
                #[cfg(debug_assertions)]
                info!("Tab button pressed: tab_index {}", tab_button.tab_index);
                
                commands.entity(button_entity).insert(TabPressed);
                
                // Handle tab switching on press
                if let Ok((mut tab_group, _tab_meta)) = q_tab_groups.get_mut(tab_button.group_entity) {
                    let previous_tab = tab_group.selected_tab;
                    if previous_tab != tab_button.tab_index {
                        #[cfg(debug_assertions)]
                        info!("Switching tab from {} to {}", previous_tab, tab_button.tab_index);
                        
                        tab_group.selected_tab = tab_button.tab_index;
                        
                        evw_tab_change.write(TabChangedEvent {
                            group_entity: tab_button.group_entity,
                            previous_tab,
                            new_tab: tab_button.tab_index,
                            change_kind: TabChangeKind::UserInteraction,
                        });
                        
                        commands.entity(tab_button.group_entity).insert(TabNeedsVisualUpdate);
                        
                        #[cfg(debug_assertions)]
                        info!("Tab switch complete, marked group for visual update");
                    } else {
                        #[cfg(debug_assertions)]
                        info!("Tab {} already selected, no change needed", tab_button.tab_index);
                    }
                } else {
                    #[cfg(debug_assertions)]
                    warn!("Could not find tab group {:?} for button", tab_button.group_entity);
                }
            }
            Interaction::Hovered => {
                commands.entity(button_entity).insert(TabHovered);
            }
            Interaction::None => {
                commands.entity(button_entity)
                    .remove::<TabHovered>()
                    .remove::<TabPressed>();
            }
        }
    }
}

pub fn tab_keyboard_navigation_system(
    mut commands: Commands,
    mut q_tab_groups: Query<(Entity, &mut TabGroup, &TabGroupMeta)>,
    q_focused_tabs: Query<(&TabButton, &TabFocused)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evw_tab_change: EventWriter<TabChangedEvent>,
) {
    for (focused_button, _) in &q_focused_tabs {
        if let Ok((group_entity, mut tab_group, tab_meta)) = q_tab_groups.get_mut(focused_button.group_entity) {
            let mut new_tab = tab_group.selected_tab;
            
            if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::ArrowUp) {
                new_tab = if new_tab == 0 { tab_meta.tab_names.len() - 1 } else { new_tab - 1 };
            } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::ArrowDown) {
                new_tab = (new_tab + 1) % tab_meta.tab_names.len();
            }
            
            if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
                new_tab = focused_button.tab_index;
            }
            
            if new_tab != tab_group.selected_tab {
                let previous_tab = tab_group.selected_tab;
                tab_group.selected_tab = new_tab;
                
                commands.entity(group_entity).insert(TabNeedsVisualUpdate);
                
                evw_tab_change.write(TabChangedEvent {
                    group_entity,
                    previous_tab,
                    new_tab,
                    change_kind: TabChangeKind::UserInteraction,
                });
            }
        }
    }
}

pub fn tab_visual_update_system(
    mut commands: Commands,
    q_tab_groups: Query<(Entity, &TabGroup, &TabGroupMeta), With<TabNeedsVisualUpdate>>,
    mut q_tab_content: Query<(&TabContent, &mut Visibility)>,
) {
    for (group_entity, tab_group, tab_meta) in q_tab_groups.iter() {
        #[cfg(debug_assertions)]
        info!("Processing visual update for tab group {:?}, selected_tab: {}", 
              group_entity, tab_group.selected_tab);
        
        let mut content_updated = 0;
        
        // Update content visibility only
        for (tab_content, mut visibility) in q_tab_content.iter_mut() {
            if tab_content.group_entity == group_entity {
                let new_visibility = if tab_content.tab_index == tab_group.selected_tab {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
                
                #[cfg(debug_assertions)]
                if *visibility != new_visibility {
                    info!("Updating content visibility: tab_index {} -> {:?}", 
                          tab_content.tab_index, new_visibility);
                }
                
                *visibility = new_visibility;
                content_updated += 1;
            }
        }
        
        #[cfg(debug_assertions)]
        info!("Updated visibility for {} content entities", content_updated);
        
        commands.entity(group_entity).remove::<TabNeedsVisualUpdate>();
    }
}

pub fn tab_content_management_system(
    mut commands: Commands,
    mut q_tab_groups: Query<(&TabGroup, &TabGroupMeta, &TabGroupConfig)>,
    q_tab_content: Query<&TabContent>,
    asset_server: Res<AssetServer>,
) {
    for (tab_group, tab_meta, config) in &mut q_tab_groups {
        match config.strategy {
            ContentStrategy::Preloaded => {
                // Content already exists, just manage visibility (handled in visual system)
            }
            ContentStrategy::LazyLoaded => {
                // Spawn content when first selected
                if let Some(content_entity) = tab_meta.content_entities.get(tab_group.selected_tab) {
                    if let Ok(tab_content) = q_tab_content.get(*content_entity) {
                        // Check if content needs to be loaded
                        // This would involve more complex state tracking
                    }
                }
            }
            ContentStrategy::Dynamic => {
                // Create/destroy content on tab changes
                // This would involve more complex entity management
            }
        }
    }
}

pub fn tab_focus_system(
    mut commands: Commands,
    q_tab_buttons: Query<(Entity, &TabButton), With<TabButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_focused_tabs: Query<Entity, With<TabFocused>>,
    q_tab_groups: Query<(&TabGroup, &TabGroupMeta)>,
    mut q_tab_buttons_with_focus: Query<(Entity, &TabButton), With<TabFocused>>,
) {
    // Keyboard-based focus navigation
    if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowRight) {
        for (focused_entity, focused_tab_button) in q_tab_buttons_with_focus.iter() {
            if let Ok((tab_group, tab_meta)) = q_tab_groups.get(focused_tab_button.group_entity) {
                let current_index = focused_tab_button.tab_index;
                let tab_count = tab_meta.tab_names.len();
                
                let new_index = if keyboard.just_pressed(KeyCode::ArrowLeft) {
                    if current_index == 0 { tab_count - 1 } else { current_index - 1 }
                } else {
                    (current_index + 1) % tab_count
                };
                
                // Find and focus the new tab
                for (entity, tab_button) in q_tab_buttons.iter() {
                    if tab_button.group_entity == focused_tab_button.group_entity 
                        && tab_button.tab_index == new_index {
                        commands.entity(entity).insert(TabFocused);
                        commands.entity(focused_entity).remove::<TabFocused>();
                        break;
                    }
                }
            }
        }
    }
    
    // Mouse-based focus would need a separate system with proper Interaction query
}

pub fn tab_continuous_visual_update_system(
    mut q_tab_buttons: Query<(
        &TabButton, 
        Option<&TabActive>, 
        Option<&TabHovered>, 
        Option<&TabPressed>, 
        Option<&TabFocused>, // Added focus
        &mut BackgroundColor, 
        &mut Transform
    )>,
    q_tab_groups: Query<&TabGroup>,
    time: Res<Time>,
) {
    for (tab_button, is_active, is_hovered, is_pressed, is_focused, mut bg_color, mut transform) in &mut q_tab_buttons {
        if let Ok(tab_group) = q_tab_groups.get(tab_button.group_entity) {
            let is_selected = tab_button.tab_index == tab_group.selected_tab;
            
            // Determine target visual state with clear priority
            let target_color = if is_selected {
                Color::srgb(0.9, 0.9, 0.9) // Active tab (light, highest priority)
            } else if is_pressed.is_some() {
                Color::srgb(0.6, 0.6, 0.6) // Pressed (darker)
            } else if is_focused.is_some() {
                Color::srgb(0.8, 0.8, 0.9) // Keyboard focused (slight blue tint)
            } else if is_hovered.is_some() {
                Color::srgb(0.75, 0.75, 0.75) // Hovered (light gray)
            } else {
                Color::srgb(0.65, 0.65, 0.65) // Inactive (medium gray)
            };
            
            let target_scale = if is_pressed.is_some() { 0.95 } else { 1.0 };
            
            // Smooth interpolation
            bg_color.0 = bg_color.0.mix(&target_color, time.delta_secs() * 10.0);
            transform.scale = transform.scale.lerp(Vec3::splat(target_scale), time.delta_secs() * 15.0);
        }
    }
}

// systems.rs (new event-driven content system)
pub fn tab_content_visibility_system(
    mut commands: Commands,
    mut evr_tab_change: EventReader<TabChangedEvent>,
    mut q_tab_content: Query<(&TabContent, &mut Visibility)>,
    q_tab_buttons: Query<(Entity, &TabButton)>,
) {
    for event in evr_tab_change.read() {
        #[cfg(debug_assertions)]
        info!("Received TabChangedEvent: group={:?}, from {} to {}", 
              event.group_entity, event.previous_tab, event.new_tab);
        
        let mut content_updated = 0;
        
        // Hide all content in this group
        for (tab_content, mut visibility) in q_tab_content.iter_mut() {
            if tab_content.group_entity == event.group_entity {
                let new_visibility = if tab_content.tab_index == event.new_tab {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
                
                #[cfg(debug_assertions)]
                if *visibility != new_visibility {
                    info!("Content visibility change: tab_index {} {:?} -> {:?}", 
                          tab_content.tab_index, *visibility, new_visibility);
                }
                
                *visibility = new_visibility;
                content_updated += 1;
            }
        }
        
        // Update tab button active/inactive markers
        for (button_entity, tab_button) in q_tab_buttons.iter() {
            if tab_button.group_entity == event.group_entity {
                if tab_button.tab_index == event.new_tab {
                    // This button should be active
                    commands.entity(button_entity).insert(TabActive).remove::<TabInactive>();
                    #[cfg(debug_assertions)]
                    info!("Set button {} (tab {}) to active", button_entity, tab_button.tab_index);
                } else {
                    // This button should be inactive
                    commands.entity(button_entity).insert(TabInactive).remove::<TabActive>();
                    #[cfg(debug_assertions)]
                    info!("Set button {} (tab {}) to inactive", button_entity, tab_button.tab_index);
                }
            }
        }
        
        #[cfg(debug_assertions)]
        info!("Updated visibility for {} content entities", content_updated);
    }
}