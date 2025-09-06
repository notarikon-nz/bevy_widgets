use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use super::components::*;

pub struct TabGroupBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    tabs: Vec<TabDefinition>,
    config: TabGroupConfig,
    initial_tab: usize,
}

pub struct TabDefinition {
    pub name: String,
    pub icon: Option<Handle<Image>>,
    pub content_builder: Option<Box<dyn FnOnce(&mut Commands) -> Entity>>,
}

impl<'w, 's, 'a> TabGroupBuilder<'w, 's, 'a> {
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            tabs: Vec::new(),
            config: TabGroupConfig {
                tab_style: TabStyle::Top,
                panel_style: Node::default(),
                animation_duration: 0.2,
                strategy: ContentStrategy::Preloaded,
                tab_button_style: Node {
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::right(Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                active_tab_style: Node::default(),
                inactive_tab_style: Node::default(),
                tab_spacing: 4.0,
            },
            initial_tab: 0,
        }
    }

    pub fn with_tab(mut self, name: impl Into<String>, content_builder: impl FnOnce(&mut Commands) -> Entity + 'static) -> Self {
        self.tabs.push(TabDefinition {
            name: name.into(),
            icon: None,
            content_builder: Some(Box::new(content_builder)),
        });
        self
    }

    pub fn with_config(mut self, config: TabGroupConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_initial_tab(mut self, index: usize) -> Self {
        self.initial_tab = index;
        self
    }

    pub fn spawn(self) -> Entity {
        let tab_names: Vec<String> = self.tabs.iter().map(|tab| tab.name.clone()).collect();
        let mut content_entities = Vec::new();
        let mut button_entities = Vec::new();

        // Create content entities first
        for (index, tab) in self.tabs.into_iter().enumerate() {
            let content_entity = if let Some(builder) = tab.content_builder {
                builder(self.commands)
            } else {
                self.commands.spawn(Node::default()).id()
            };

            self.commands.entity(content_entity).insert((
                TabContent {
                    tab_index: index,
                    group_entity: Entity::PLACEHOLDER, // Will be updated later
                },
                Visibility::Hidden,
            ));

            content_entities.push(content_entity);
        }

        // Create tab group with all metadata
        let group_entity = self.commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            TabGroup {
                selected_tab: self.initial_tab,
            },
            TabGroupMeta {
                tab_names: tab_names.clone(),
                content_entities: content_entities.clone(),
                button_entities: Vec::new(), // Will be filled below
            },
            self.config.clone(),
            TabNeedsVisualUpdate,
        )).id();

        // Update content entities with correct group entity reference
        for content_entity in &content_entities {
            self.commands.entity(*content_entity).insert(TabContent {
                tab_index: 0, // Will be set correctly in the loop
                group_entity,
            });
        }

        // Spawn tab bar
        let tab_bar_entity = self.commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
        )).id();

        // Spawn tab buttons
        for (index, tab_name) in tab_names.iter().enumerate() {
            let is_active = index == self.initial_tab;
            
            let tab_button_entity = self.commands.spawn((
                Button,
                self.config.tab_button_style.clone(),
                BackgroundColor(if is_active {
                    Color::srgb(0.7, 0.7, 0.7)
                } else {
                    Color::srgb(0.5, 0.5, 0.5)
                }),
                TabButton {
                    tab_index: index,
                    group_entity,
                },
                TabInteractionState::default(),
                FocusPolicy::Block,
            )).id();

            // Add state markers
            if is_active {
                self.commands.entity(tab_button_entity).insert(TabActive);
            } else {
                self.commands.entity(tab_button_entity).insert(TabInactive);
            }

            // Add text to button
            self.commands.entity(tab_button_entity).with_children(|parent| {
                parent.spawn((
                    Text::new(tab_name),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            button_entities.push(tab_button_entity);
            self.commands.entity(tab_bar_entity).add_child(tab_button_entity);
        }

        // Update tab group with button entities
        self.commands.entity(group_entity).insert(TabGroupMeta {
            tab_names: tab_names.clone(),
            content_entities: content_entities.clone(),
            button_entities: button_entities.clone(),
        });

        // Spawn content panel
        let panel_entity = self.commands.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                ..default()
            },
            TabPanel,
        )).id();

        // Add content to panel
        for content_entity in &content_entities {
            self.commands.entity(panel_entity).add_child(*content_entity);
        }

        // Set initial content visibility
        if let Some(initial_content) = content_entities.get(self.initial_tab) {
            self.commands.entity(*initial_content).insert(Visibility::Visible);
        }

        self.commands.entity(group_entity)
            .add_children(&[tab_bar_entity, panel_entity]);

        group_entity
    }
}