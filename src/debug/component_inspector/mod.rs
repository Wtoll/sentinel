//! A component inspector


use bevy::{camera::RenderTarget, ecs::schedule::ScheduleLabel, prelude::*, window::WindowRef};
use bevy_egui::{EguiContext, EguiGlobalSettings, EguiMultipassSchedule, EguiPlugin};
use ratatui::{Terminal, backend::Backend, layout::Size, widgets::Paragraph};
use soft_ratatui::{EmbeddedGraphics, SoftBackend, embedded_graphics_unicodefonts::{mono_8x13_atlas, mono_8x13_bold_atlas, mono_8x13_italic_atlas}};
use egui::{TextureHandle, TextureOptions, Widget};

pub struct ComponentInspectorPlugin;

impl Plugin for ComponentInspectorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin::default())
            .insert_resource(EguiGlobalSettings {
                ..default()
            })
            .add_systems(Startup, startup)
            .add_systems(DebugInspectorPass, egui_pass);
    }
}

#[derive(Component)]
pub struct DebugInspector;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DebugInspectorPass;


fn startup(
    mut commands: Commands
) -> Result {

    let inspector_window = commands.spawn((
        Window {
            title: String::from("Debug Inspector"),
            ..default()
        }
    )).id();

    commands.spawn((
        DebugInspector,
        Camera3d::default(),
        Camera::default(),
        RenderTarget::Window(WindowRef::Entity(inspector_window)),
        EguiMultipassSchedule::new(DebugInspectorPass),
        RataguiContext::default()
    ));

    Ok(())
}




fn egui_pass(
    mut context: Single<(&mut EguiContext, &mut RataguiContext), With<DebugInspector>>
) -> Result {

    let (mut egui_context, mut ratagui_context) = context.into_inner();

    ratagui_context.terminal_mut().draw(|frame| {
        let area = frame.area();
        frame.render_widget(Paragraph::new("This is some text"), area);
    }).expect("Failure");


    let panel = egui::CentralPanel::default()
        .frame(egui::Frame::NONE);

    panel.show(egui_context.get_mut(), |ui| {
        ui.add(ratagui_context.as_mut())
    });

    egui::Window::new("Hello").show(egui_context.get_mut(), |ui| {
        ui.label("world");
    });

    Ok(())
}
















#[derive(Component)]
pub struct RataguiContext {
    terminal: Terminal<SoftBackend<EmbeddedGraphics>>,
    size: egui::Vec2,
    tex_name: String,
    tex_handle: Option<TextureHandle>
}

impl RataguiContext {
    fn new() -> Self {

        let font_reg = mono_8x13_atlas();
        let font_bold = mono_8x13_bold_atlas();
        let font_ital = mono_8x13_italic_atlas();

        let terminal = Terminal::new(SoftBackend::<EmbeddedGraphics>::new(
            100,
            50,
            font_reg,
            Some(font_bold),
            Some(font_ital)
        )).unwrap();

        Self {
            terminal,
            size: egui::Vec2::new(0.0, 0.0),
            tex_name: String::from("yuh"),
            tex_handle: None
        }
    }

    fn resize_backend(&mut self, size: egui::Vec2) {
        self.size = size;

        let char_counts = size / egui::Vec2::new(self.backend().char_width as f32, self.backend().char_height as f32);
        let new = Size::new(char_counts.x as u16, char_counts.y as u16);

        let current = self.backend().size().unwrap(); // SAFETY: Infallible

        if new != current {
            self.backend_mut().resize(new.width, new.height);
        }
    }

    fn terminal_mut(&mut self) -> &mut Terminal<SoftBackend<EmbeddedGraphics>> {
        &mut self.terminal
    }

    fn backend(&self) -> &SoftBackend<EmbeddedGraphics> {
        self.terminal.backend()
    }

    fn backend_mut(&mut self) -> &mut SoftBackend<EmbeddedGraphics> {
        self.terminal.backend_mut()
    }
}

impl Widget for &mut RataguiContext {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let av_size = ui.available_size();

        if self.size != av_size {
            self.resize_backend(av_size);
        }

        let image = egui::ColorImage::from_rgb([self.backend().get_pixmap_width(), self.backend().get_pixmap_height()], self.backend().get_pixmap_data());

        let tex = ui.ctx().load_texture(&self.tex_name, image, TextureOptions::NEAREST);

        self.tex_handle = Some(tex.clone());

        ui.image((tex.id(), tex.size_vec2()))
    }
}

impl Default for RataguiContext {
    fn default() -> Self {
        Self::new()
    }
}
