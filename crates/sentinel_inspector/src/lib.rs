#![forbid(missing_docs)]
//! A debug inspector for Bevy
//! 


use std::collections::BTreeMap;
use std::default::Default;

use bevy::app::{App, AppLabel, Last, Plugin, Startup, Update};
use bevy::camera::{Camera, Camera3d};
use bevy::ecs::component::{Component, ComponentId};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageReader;
use bevy::ecs::name::Name;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::ecs::world::World;
use bevy::input::{ButtonInput, ButtonState};
use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
use bevy::log::*;
use bevy::platform::collections::HashMap;
use bevy::utils::default;
use bevy::window::Window;
use bevy::{camera::RenderTarget, ecs::schedule::ScheduleLabel, window::WindowRef};
use bevy_egui::{EguiContext, EguiGlobalSettings, EguiMultipassSchedule, EguiPlugin};
use ratatui::widgets::{Block, List, ListState, Widget};

use ratatui::prelude::*;

mod render;
use render::RataguiContext;









/// Plugin for enabling the debug inspector
#[derive(Default)]
pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_plugins(EguiPlugin::default())
            .init_resource::<InspectorCache>()
            .insert_resource(EguiGlobalSettings {
                ..default()
            })
            .add_systems(Startup, startup)
            .add_systems(Update, batch_queries)
            .add_systems(Last, extract_state)
            .add_systems(DebugInspectorPass, render_inspectors)
            .add_systems(Update, listen_for_keys);
    }
}

/// A schedule for the inspector rendering
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DebugInspectorPass;

/// A debug inspector window
#[derive(Component)]
pub struct DebugInspectorWindow;

fn startup(
    mut commands: Commands
) {

    let inspector_window = commands.spawn((
        Name::new("Debug Inspector Window"),
        Window {
            title: String::from("Debug Inspector"),
            ..default()
        },
        DebugInspectorWindow
    )).id();

    commands.spawn((
        Name::new("Debug Inspector"),
        DebugInspector::default(),
        Camera3d::default(),
        Camera::default(),
        RenderTarget::Window(WindowRef::Entity(inspector_window)),
        EguiMultipassSchedule::new(DebugInspectorPass),
        RataguiContext::default()
    ));
}



#[derive(Resource, Default)]
struct InspectorCache {
    /// Believe it or not, size_of::<Entity>::() == size_of::<u64>::() so this
    /// is equally performant for additional information.
    query_ranges: Vec<(Entity, Entity)>, 
    query_entities: Vec<Entity>,
    entity_data: BTreeMap<Entity, InspectorEntityData>
}


struct InspectorEntityData {
    name: Option<String>,
    components: BTreeMap<ComponentId, String>
}

impl InspectorEntityData {

    pub fn format_line(&self, id: &Entity) -> String {
        if let Some(name) = &self.name {
            format!("{} ({})", id, name)
        } else {
            format!("{}", id)
        }
    }
}



impl InspectorCache {

    fn clear_queries(&mut self) {
        self.query_ranges.clear();
        self.query_entities.clear();
    }

    fn add_query(&mut self, inspector: &DebugInspector) {
        if let Some(entity) = inspector.entity_selected {
            self.query_entities.push(entity);
        }
    }
}


















/// A debug inspector
#[derive(Component, Default)]
pub struct DebugInspector {
    height: usize,
    entity_offset: usize,
    entity_selected: Option<Entity>,
    component_offset: usize,
    component_selected: Option<ComponentId>
}

impl DebugInspector {
    fn construct_widget(&mut self, cache: &InspectorCache) -> DebugInspectorWidget {

        let mut entity_lines = Vec::new();
        let mut entity_selected = None;

        let mut component_lines = Vec::new();
        let mut component_selected = None;

        if self.entity_selected.is_none()
            && let Some((entity, _)) = cache.entity_data.first_key_value() {
                self.entity_selected = Some(*entity);
            }

        for (i, (entity, data)) in cache.entity_data.iter().enumerate() {
            entity_lines.push(data.format_line(entity));

            if Some(*entity) == self.entity_selected {
                entity_selected = Some(i);
            }
        }

        if let Some(entity) = self.entity_selected
            && let Some(entity_data) = cache.entity_data.get(&entity) {

                if self.component_selected.is_none()
                    && let Some((component, _)) = entity_data.components.first_key_value() {
                        self.component_selected = Some(*component);
                    }

                for (i, (component, data)) in entity_data.components.iter().enumerate() {
                    component_lines.push(data.clone());

                    if Some(*component) == self.component_selected {
                        component_selected = Some(i);
                    }
                }
            }
        
        DebugInspectorWidget {
            entity_lines,
            entity_selected,
            component_lines,
            component_selected
        }
    }
}








fn batch_queries(
    inspectors: Query<&DebugInspector>,
    mut inspector_cache: ResMut<InspectorCache>
) {
    inspector_cache.clear_queries();

    inspectors.iter().for_each(|inspector| {
        inspector_cache.add_query(inspector);
    });
}

fn extract_state(
    world: &mut World
) {
    // SAFETY: Simultaneous access is only performed for disjoint parts of the world
    unsafe {
        let unsafe_world = world.as_unsafe_world_cell();

        let mut inspector_cache = unsafe_world.get_resource_mut::<InspectorCache>().unwrap();

        let mut entity_query = unsafe_world.world_mut().query::<(Entity, Option<&Name>)>();

        entity_query.iter(unsafe_world.world())
            .for_each(|(e, n)| {
                if let Some(data) = inspector_cache.entity_data.get_mut(&e) {
                    data.name = n.map(|name| name.to_string());
                } else {
                    inspector_cache.entity_data.insert(e, InspectorEntityData {
                        name: n.map(|name| name.to_string()),
                        components: BTreeMap::new()
                    });
                }
            });


        for entity in inspector_cache.query_entities.clone().into_iter() {

            if let Ok(components) = unsafe_world.world().inspect_entity(entity) {

                inspector_cache.entity_data.entry(entity)
                    .or_insert_with(|| InspectorEntityData {
                        name: None,
                        components: BTreeMap::new()
                    });

                let data = inspector_cache.entity_data.get_mut(&entity).unwrap();

                components.for_each(|component| {
                    data.components.insert(component.id(), component.name().to_string());
                });
            }

        }
    }
}




struct DebugInspectorWidget {
    entity_lines: Vec<String>,
    entity_selected: Option<usize>,
    component_lines: Vec<String>,
    component_selected: Option<usize>,
}

impl ratatui::widgets::Widget for DebugInspectorWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {

        // Create layout

        let [entities_area, components_area, data_area] = Layout::horizontal([Constraint::Percentage(33); 3]).areas(area);

        // Render entities area

        {
            let mut list_state = ListState::default()
                .with_selected(self.entity_selected);

            let list = List::new(self.entity_lines)
                .highlight_style(Style::new().black().on_white())
                .block(Block::bordered());

            StatefulWidget::render(list, entities_area, buf, &mut list_state);
        }

        // Render components area

        {
            let mut list_state = ListState::default();

            let list = List::new(self.component_lines)
                .highlight_style(Style::new().black().on_white())
                .block(Block::bordered());

            StatefulWidget::render(list, components_area, buf, &mut list_state);
        }

    }
}





fn render_inspectors(
    inspectors: Query<(&mut DebugInspector, &mut EguiContext, &mut RataguiContext)>,
    inspector_cache: Res<InspectorCache>
) {
    for (mut inspector, mut egui_context, mut ratagui_context) in inspectors {
        render::combine(egui_context.as_mut(), ratagui_context.as_mut(), inspector.as_mut().construct_widget(&inspector_cache))
    }
}














fn listen_for_keys(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut inspectors: Query<(&mut DebugInspector, &RenderTarget)>,
    mut inspector_cache: ResMut<InspectorCache>
) {
    for keyboard_event in keyboard_events.read() {
        if let KeyboardInput {
            key_code,
            state: ButtonState::Pressed,
            window,
            ..
        } = keyboard_event {
            for (mut inspector, target) in inspectors.iter_mut() {
                if let Some(selected) = inspector.entity_selected && let RenderTarget::Window(WindowRef::Entity(target)) = target && target == window {
                    match key_code {
                        KeyCode::KeyS | KeyCode::ArrowDown => {
                            if let Some(next) = inspector_cache.entity_data.keys()
                                .skip_while(|entity| **entity != selected).nth(1) {
                                    inspector.entity_selected = Some(*next);
                                }
                        },
                        KeyCode::KeyW | KeyCode::ArrowUp => {
                            if let Some(next) = inspector_cache.entity_data.keys()
                                .rev()
                                .skip_while(|entity| **entity != selected).nth(1) {
                                    inspector.entity_selected = Some(*next);
                                }
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}