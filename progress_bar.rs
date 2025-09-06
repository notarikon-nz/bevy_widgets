use bevy::prelude::*;
use std::{collections::HashMap, sync::Arc};

// =============================================================================
// BUNDLES

#[derive(Bundle)]
pub struct ProgressBarBundle {
    pub progress: ProgressBar,
    pub visuals: ProgressBarVisuals,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub root: ProgressBarRoot,
}

impl ProgressBarBundle {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            progress: ProgressBar::new(min, max),
            visuals: ProgressBarVisuals::default(),
            node: Node::default(),
            background_color: BackgroundColor::default(),
            root: ProgressBarRoot,
        }
    }
    
    pub fn with_value(mut self, value: f32) -> Self {
        self.progress.set_value(value);
        self
    }
    
    pub fn with_visuals(mut self, visuals: ProgressBarVisuals) -> Self {
        self.visuals = visuals;
        self
    }
    
    pub fn with_size(mut self, width: Val, height: Val) -> Self {
        self.node.width = width;
        self.node.height = height;
        self
    }
}

// =============================================================================
// COMPONENTS
// =============================================================================

#[derive(Component, Debug, Clone, Reflect)]
pub struct ProgressBar {
    pub current: f32,
    pub max: f32,
    pub min: f32,
    pub previous_value: f32, // For threshold detection
}

impl ProgressBar {
    pub fn new(current: f32, max: f32) -> Self {
        Self {
            current,
            max,
            min: 0.0,
            previous_value: current,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.previous_value = self.current;
        self.current = value.clamp(self.min, self.max);
    }

    pub fn fraction(&self) -> f32 {
        ((self.current - self.min) / (self.max - self.min).max(0.01)).clamp(0.0, 1.0)
    }
    
    pub fn percentage(&self) -> f32 {
        self.fraction() * 100.0
    }
}

#[derive(Component, Clone)]
pub struct ProgressBarVisuals {
    pub orientation: ProgressOrientation,
    pub fill_direction: FillDirection,
    pub show_text: bool,
    pub text_format: ProgressTextFormat,
    pub track_color: Color,
    pub fill_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub border_width: f32,
    pub fill_margin: f32,
    pub animation_duration: f32,
    pub easing: EasingFunction,
}

impl Default for ProgressBarVisuals {
    fn default() -> Self {
        Self {
            orientation: ProgressOrientation::Horizontal,
            fill_direction: FillDirection::LeftToRight,
            show_text: true,
            text_format: ProgressTextFormat::Percentage,
            track_color: Color::srgb(0.1, 0.1, 0.1),
            fill_color: Color::srgb(0.2, 0.6, 0.2),
            border_color: Color::srgb(0.3, 0.3, 0.3),
            text_color: Color::WHITE,
            border_width: 2.0,
            fill_margin: 1.0,
            animation_duration: 0.3, // seconds to complete animation
            easing: EasingFunction::EaseOut,
        }
    }
}

impl std::fmt::Debug for ProgressBarVisuals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgressBarVisuals")
            .field("orientation", &self.orientation)
            .field("fill_direction", &self.fill_direction)
            .field("show_text", &self.show_text)
            .field("text_format", &self.text_format)
            .field("track_color", &self.track_color)
            .field("fill_color", &self.fill_color)
            .field("border_color", &self.border_color)
            .field("text_color", &self.text_color)
            .field("border_width", &self.border_width)
            .field("fill_margin", &self.fill_margin)
            .field("animation_duration", &self.animation_duration)
            .field("easing", &self.easing)
            .finish()
    }
}

#[derive(Component, Debug, Default)]
pub struct ProgressAnimation {
    pub target_fraction: f32,
    pub current_display_fraction: f32,
    pub start_fraction: f32, // Where the animation started from
    pub start_time: f64,
    pub duration: f32,
    pub is_animating: bool,
}

#[derive(Component)]
pub struct ProgressBarParts {
    pub fill: Entity,
    pub text: Option<Entity>,
}

// Marker components
#[derive(Component)]
pub struct ProgressBarRoot;

#[derive(Component)]
pub struct ProgressBarTrack;

#[derive(Component)]
pub struct ProgressBarFill;

#[derive(Component)]
pub struct ProgressBarText;

#[derive(Component)]
pub struct ProgressBarBorder;

#[derive(Component)]
pub struct ProgressCompleted;

// =============================================================================
// ENUMS & TYPES
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum ProgressOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum FillDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

impl EasingFunction {
    pub fn sample(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingFunction::Bounce => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t = t - 1.5 / 2.75;
                    7.5625 * t * t + 0.75
                } else if t < 2.5 / 2.75 {
                    let t = t - 2.25 / 2.75;
                    7.5625 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / 2.75;
                    7.5625 * t * t + 0.984375
                }
            }
            EasingFunction::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                    let result = -2.0f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin();
                    result.clamp(0.0, 1.0) // Prevent negative overshoot
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum ProgressTextFormat {
    Percentage,
    Fraction,
    Absolute,
    Custom(Arc<dyn Fn(f32, f32) -> String + Send + Sync>),
}

impl std::fmt::Debug for ProgressTextFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgressTextFormat::Percentage => write!(f, "ProgressTextFormat::Percentage"),
            ProgressTextFormat::Fraction => write!(f, "ProgressTextFormat::Fraction"),
            ProgressTextFormat::Absolute => write!(f, "ProgressTextFormat::Absolute"),
            ProgressTextFormat::Custom(_) => write!(f, "ProgressTextFormat::Custom(<closure>)"),
        }
    }
}

impl ProgressTextFormat {
    pub fn format(&self, current: f32, max: f32) -> String {
        match self {
            ProgressTextFormat::Percentage => format!("{:.0}%", (current / max.max(0.01)) * 100.0),
            ProgressTextFormat::Fraction => format!("{:.0}/{:.0}", current, max),
            ProgressTextFormat::Absolute => format!("{:.1}/{:.1}", current, max),
            ProgressTextFormat::Custom(func) => func(current, max),
        }
    }
}

// =============================================================================
// EVENTS
// =============================================================================

#[derive(Event, Debug, Clone, Copy)]
pub struct ProgressChangedEvent {
    pub progress_entity: Entity,
    pub previous_value: f32,
    pub new_value: f32,
    pub change_kind: ProgressChangeKind,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ProgressThresholdEvent {
    pub progress_entity: Entity,
    pub threshold: f32,
    pub crossed_up: bool,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ProgressCompletedEvent {
    pub progress_entity: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressChangeKind {
    Incremental,
    Direct,
    Reset,
    Complete,
}

// =============================================================================
// RESOURCES
// =============================================================================

#[derive(Resource, Default)]
pub struct ProgressThresholds(pub Vec<f32>);

#[derive(Resource, Default)]
pub struct ProgressBarAssets {
    pub default_font: Handle<Font>,
}

// =============================================================================
// SYSTEMS
// =============================================================================

pub fn progress_bar_update_system(
    mut commands: Commands,
    mut q_progress_bars: Query<(
        Entity,
        &ProgressBar,
        &ProgressBarVisuals,
        &ProgressBarParts,
        Option<&mut ProgressAnimation>,
    ), (With<ProgressBarRoot>, Or<(Changed<ProgressBar>, Added<ProgressBar>)>)>,
    mut q_fills: Query<&mut Node, With<ProgressBarFill>>,
    mut evw_changed: EventWriter<ProgressChangedEvent>,
    time: Res<Time>,
) {
    
    for (entity, progress, visuals, parts, animation) in &mut q_progress_bars {
        let target_fraction = progress.fraction();
        
        // Send change event
        evw_changed.write(ProgressChangedEvent {
            progress_entity: entity,
            previous_value: progress.previous_value,
            new_value: progress.current,
            change_kind: determine_change_kind(&progress),
        });
        if let Ok(mut fill_style) = q_fills.get_mut(parts.fill) {
            if let Some(mut animation) = animation {
                // Use fixed duration from visuals
                animation.duration = visuals.animation_duration;
                // Record where this animation is starting from
                animation.start_fraction = animation.current_display_fraction;
                animation.target_fraction = target_fraction;
                animation.start_time = time.elapsed_secs_f64();
                animation.is_animating = true;
            } else {
                // Immediate update (no animation component) - convert fraction to percentage for styling
                let target_percentage = calculate_fill_percentage(progress, visuals);
                update_fill_style(&mut fill_style, target_percentage, visuals);
            }
        } else {
            warn!("Could not find fill entity {:?} in q_fills query", parts.fill);
        }
        
        // Mark as completed if at max
        if progress.current >= progress.max {
            commands.entity(entity).insert(ProgressCompleted);
        }
    }
}

pub fn progress_bar_animation_system(
    _commands: Commands,
    time: Res<Time>,
    mut q_animations: Query<(
        Entity,
        &ProgressBar,
        &ProgressBarVisuals,
        &ProgressBarParts,
        &mut ProgressAnimation,
    )>,
    mut q_fills: Query<&mut Node, With<ProgressBarFill>>,
    mut evw_completed: EventWriter<ProgressCompletedEvent>,
) {
    let current_time = time.elapsed_secs_f64();
    
    for (entity, progress, visuals, parts, mut animation) in &mut q_animations {
        if animation.is_animating {
            let elapsed = (current_time - animation.start_time) as f32;
            let t = (elapsed / animation.duration.max(0.001)).clamp(0.0, 1.0);
            
            let eased_t = visuals.easing.sample(t);
            
            // Proper lerp from start_fraction to target_fraction
            animation.current_display_fraction = animation.start_fraction.lerp(animation.target_fraction, eased_t);
            
            // Update fill - convert fraction to percentage for styling
            if let Ok(mut fill_style) = q_fills.get_mut(parts.fill) {
                let display_percentage = match visuals.fill_direction {
                    FillDirection::RightToLeft | FillDirection::BottomToTop => (1.0 - animation.current_display_fraction) * 100.0,
                    _ => animation.current_display_fraction * 100.0,
                };
                update_fill_style(&mut fill_style, display_percentage, visuals);
            } else {
                warn!("animation_system: Could not find fill entity {:?}", parts.fill);
            }
            
            // Check if animation complete
            if t >= 1.0 {
                animation.is_animating = false;
                animation.current_display_fraction = animation.target_fraction;
                
                // Fire completion event if reached max
                if progress.current >= progress.max {
                    evw_completed.write(ProgressCompletedEvent {
                        progress_entity: entity,
                    });
                }
            }
        }
    }
}

pub fn progress_bar_text_system(
    mut q_texts: Query<&mut Text, With<ProgressBarText>>,
    q_progress_bars: Query<(&ProgressBar, &ProgressBarVisuals, &ProgressBarParts), 
        Or<(Changed<ProgressBar>, Changed<ProgressBarVisuals>)>>,
    mut last_values: Local<HashMap<Entity, i32>>,
) {
    for (progress, visuals, parts) in &q_progress_bars {
        if visuals.show_text && parts.text.is_some() {
            if let Ok(mut text) = q_texts.get_mut(parts.text.unwrap()) {
                let current_int = (progress.current * 10.0) as i32; // 0.1 precision
                
                // Only update if value changed significantly
                if last_values.get(&parts.text.unwrap()) != Some(&current_int) {
                    text.0 = visuals.text_format.format(progress.current, progress.max);
                    last_values.insert(parts.text.unwrap(), current_int);
                }
            }
        }
    }
}

pub fn progress_threshold_system(
    q_progress_bars: Query<(Entity, &ProgressBar), Changed<ProgressBar>>,
    mut evw_threshold: EventWriter<ProgressThresholdEvent>,
    thresholds: Res<ProgressThresholds>,
) {
    for (entity, progress) in &q_progress_bars {
        let previous_normalized = (progress.previous_value - progress.min) / (progress.max - progress.min);
        let current_normalized = (progress.current - progress.min) / (progress.max - progress.min);
        
        for &threshold in thresholds.0.iter() {
            if previous_normalized < threshold && current_normalized >= threshold {
                evw_threshold.write(ProgressThresholdEvent {
                    progress_entity: entity,
                    threshold,
                    crossed_up: true,
                });
            } else if previous_normalized >= threshold && current_normalized < threshold {
                evw_threshold.write(ProgressThresholdEvent {
                    progress_entity: entity,
                    threshold,
                    crossed_up: false,
                });
            }
        }
    }
}

#[cfg(debug_assertions)]
pub fn progress_bar_contrast_check_system(
    q_progress_bars: Query<&ProgressBarVisuals, Added<ProgressBarVisuals>>,
) {
    for visuals in &q_progress_bars {
        let text_luminance = calculate_luminance(visuals.text_color);
        let fill_luminance = calculate_luminance(visuals.fill_color);
        let track_luminance = calculate_luminance(visuals.track_color);
        
        let contrast_ratio_fill = (text_luminance.max(fill_luminance) + 0.05) 
            / (text_luminance.min(fill_luminance) + 0.05);
        let contrast_ratio_track = (text_luminance.max(track_luminance) + 0.05) 
            / (text_luminance.min(track_luminance) + 0.05);
        
        if contrast_ratio_fill < 4.5 {
            warn!("Progress bar text may have poor contrast against fill color (ratio: {:.2})", contrast_ratio_fill);
        }
        if contrast_ratio_track < 4.5 {
            warn!("Progress bar text may have poor contrast against track color (ratio: {:.2})", contrast_ratio_track);
        }
    }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

fn calculate_fill_percentage(progress: &ProgressBar, visuals: &ProgressBarVisuals) -> f32 {
    let fraction = progress.fraction();
    
    match visuals.fill_direction {
        FillDirection::RightToLeft | FillDirection::BottomToTop => (1.0 - fraction) * 100.0,
        _ => fraction * 100.0,
    }
}

fn update_fill_style(style: &mut Node, fill_percentage: f32, visuals: &ProgressBarVisuals) {
    match visuals.orientation {
        ProgressOrientation::Horizontal => {
            style.width = Val::Percent(fill_percentage);
        }
        ProgressOrientation::Vertical => {
            style.height = Val::Percent(fill_percentage);
        }
    }
}

fn determine_change_kind(progress: &ProgressBar) -> ProgressChangeKind {
    if progress.current >= progress.max {
        ProgressChangeKind::Complete
    } else if progress.current <= progress.min {
        ProgressChangeKind::Reset
    } else if (progress.current - progress.previous_value).abs() > 10.0 {
        ProgressChangeKind::Direct
    } else {
        ProgressChangeKind::Incremental
    }
}

fn calculate_luminance(color: Color) -> f32 {
    let srgba = color.to_srgba();
    0.2126 * srgba.red + 0.7152 * srgba.green + 0.0722 * srgba.blue
}

// =============================================================================
// BUILDER
// =============================================================================

pub struct ProgressBarBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    initial_value: f32,
    max_value: f32,
    visuals: ProgressBarVisuals,
    size: (Val, Val),
    with_animation: bool,
}

impl<'w, 's, 'a> ProgressBarBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            initial_value: 0.0,
            max_value: 100.0,
            visuals: ProgressBarVisuals::default(),
            size: (Val::Px(200.0), Val::Px(20.0)),
            with_animation: true,
        }
    }

    pub fn with_value(mut self, current: f32, max: f32) -> Self {
        self.initial_value = current;
        self.max_value = max;
        self
    }

    pub fn with_visuals(mut self, visuals: ProgressBarVisuals) -> Self {
        self.visuals = visuals;
        self
    }

    pub fn with_size(mut self, width: Val, height: Val) -> Self {
        self.size = (width, height);
        self
    }

    pub fn with_animation(mut self, enabled: bool) -> Self {
        self.with_animation = enabled;
        self
    }

    pub fn spawn(mut self) -> Entity {
        self.spawn_internal(None)
    }

    pub fn insert(mut self, parent: Entity) -> Entity {
        let entity = self.spawn_internal(None);
        self.commands.entity(parent).add_child(entity);
        entity
    }

    fn spawn_internal(&mut self, _parent: Option<Entity>) -> Entity {
        // Calculate initial fill percentage
        let initial_percentage = calculate_fill_percentage(
            &ProgressBar::new(self.initial_value, self.max_value),
            &self.visuals
        );

        // Spawn root container
        let root_entity = self.commands.spawn((
            Node {
                width: self.size.0,
                height: self.size.1,
                ..default()
            },
            ProgressBarRoot,
            ProgressBar::new(self.initial_value, self.max_value),
            self.visuals.clone(),
        )).id();

        // Spawn border
        let border_entity = self.commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(self.visuals.border_width)),
                ..default()
            },
            BorderColor(self.visuals.border_color),
            ProgressBarBorder,
        )).id();

        // Spawn track
        let track_entity = self.commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(self.visuals.fill_margin)),
                ..default()
            },
            BackgroundColor(self.visuals.track_color),
            ProgressBarTrack,
        )).id();

        // Spawn fill
        let fill_node = match self.visuals.orientation {
            ProgressOrientation::Horizontal => Node {
                width: Val::Percent(initial_percentage.max(10.0)), // Ensure minimum visibility for debugging
                height: Val::Percent(100.0),
                ..default()
            },
            ProgressOrientation::Vertical => Node {
                width: Val::Percent(100.0),
                height: Val::Percent(initial_percentage.max(10.0)), // Ensure minimum visibility for debugging
                ..default()
            },
        };

        let fill_entity = self.commands.spawn((
            fill_node,
            BackgroundColor(self.visuals.fill_color),
            ProgressBarFill,
        )).id();

        // Spawn text (optional)
        let text_entity = if self.visuals.show_text {
            let text_entity = self.commands.spawn((
                Text::new(self.visuals.text_format.format(self.initial_value, self.max_value)),
                TextColor(self.visuals.text_color),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ProgressBarText,
            )).id();
            Some(text_entity)
        } else {
            None
        };

        // Build hierarchy: Root -> Border -> Track -> Fill, Text as sibling to Border
        self.commands.entity(track_entity).add_child(fill_entity);
        self.commands.entity(border_entity).add_child(track_entity);
        
        let mut children = vec![border_entity];
        if let Some(text_entity) = text_entity {
            children.push(text_entity);
        }
        self.commands.entity(root_entity).add_children(&children);

        // Add parts and animation components
        self.commands.entity(root_entity).insert(ProgressBarParts {
            fill: fill_entity,
            text: text_entity,
        });

        if self.with_animation {
            let initial_fraction = (initial_percentage / 100.0).clamp(0.0, 1.0);
            self.commands.entity(root_entity).insert(ProgressAnimation {
                target_fraction: initial_fraction,
                current_display_fraction: initial_fraction,
                start_time: 0.0,
                start_fraction: 0.0,
                duration: self.visuals.animation_duration, // Initialize with proper duration!
                is_animating: false,
            });
        }

        root_entity
    }
}

// =============================================================================
// PLUGIN
// =============================================================================

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ProgressBar>()
            .register_type::<ProgressOrientation>()
            .register_type::<FillDirection>()
            .register_type::<EasingFunction>()
            .add_event::<ProgressChangedEvent>()
            .add_event::<ProgressThresholdEvent>()
            .add_event::<ProgressCompletedEvent>()
            .init_resource::<ProgressThresholds>()
            .init_resource::<ProgressBarAssets>()
            .add_systems(Update, (
                progress_bar_update_system,
                progress_bar_animation_system,
                progress_bar_text_system,
                progress_threshold_system,
            ).chain());

        #[cfg(debug_assertions)]
        app.add_systems(Update, progress_bar_contrast_check_system);
    }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

pub fn set_progress_value(
    entity: Entity,
    value: f32,
    _commands: &mut Commands,
    query: &mut Query<&mut ProgressBar>,
) {
    if let Ok(mut progress) = query.get_mut(entity) {
        progress.set_value(value);
    }
}

pub fn increment_progress(
    entity: Entity,
    amount: f32,
    _commands: &mut Commands,
    query: &mut Query<&mut ProgressBar>,
) {
    if let Ok(mut progress) = query.get_mut(entity) {
        let current = progress.current;
        progress.set_value(current + amount);
    }
}