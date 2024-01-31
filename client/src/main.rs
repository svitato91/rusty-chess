use std::sync::mpsc::channel;
use std::thread;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{PresentMode, WindowResized};

use crate::board::build_board;
use crate::connection::handle_connection;

const BASE_WIDTH: f32 = 1280.0;
const BASE_HEIGHT: f32 = 720.0;

mod board;
mod connection;
mod error;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }).set(LogPlugin {
            filter: "info,naga=warn,wgpu=error,rusty_chess=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize);

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let layout = Layout::new();

    let (s, r) = channel();
    thread::spawn(move || handle_connection(s));

    // Screen
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(BASE_WIDTH, BASE_HEIGHT)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::GRAY)),
        ..default()
    });

    build_board(&mut commands, &mut meshes, &mut materials, &layout);

    // Moves
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(layout.moves_area.0).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
        transform: Transform::from_translation(layout.moves_area.1),
        ..default()
    });
}

struct Layout {
    board_area: (Vec2, Vec3),
    moves_area: (Vec2, Vec3),
}

impl Layout {
    fn new() -> Self {
        Self {
            board_area: (Vec2::new(710., 710.), Vec3::new(-280., 0., 0.1)),
            moves_area: (Vec2::new(550., 710.), Vec3::new(360., 0., 0.1)),
        }
    }
}

// On resize keep aspect ratio
fn on_resize(
    mut projection: Query<&mut OrthographicProjection>,
    mut event_reader: EventReader<WindowResized>,
) {
    let mut projection = projection.single_mut();
    for e in event_reader.read() {
        let scale_factor = compute_scale_factor((e.width, e.height));
        debug!("Scaling projection to fit resize: {scale_factor}");
        projection.scale = scale_factor;
    }
}

fn compute_scale_factor((width, height): (f32, f32)) -> f32 {
    if width == 0.0 || height == 0.0 {
        return 1.0;
    }
    let width_scale = BASE_WIDTH / width;
    let height_scale = BASE_HEIGHT / height;
    width_scale.max(height_scale)
}
