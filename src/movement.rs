use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::util::{ AppState, Sec };

// ---- PLUGIN ----
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn name(&self) -> &str {
    "Movement Screen"
  }

  fn build(&self, app: &mut App) {
    app.add_plugin(FrameTimeDiagnosticsPlugin)
      .add_system(setup.in_schedule(OnEnter(AppState::Movement)))
      .add_system(print_fps.run_if(in_state(AppState::Movement)))
      .add_system(update.run_if(in_state(AppState::Movement)))
      .add_system(end.in_schedule(OnExit(AppState::Movement)));
  }
}

// ---- RESOURCES ----


// ---- COMPONENTS ----
#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct FpsText;

// ---- SYSTEMS ----
fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut _meshes: ResMut<Assets<Mesh>>,
  mut _materials: ResMut<Assets<ColorMaterial>>,
) {
  // spawn camera
  commands.spawn(Camera2dBundle::default());

  // spawn UI
  commands.spawn(NodeBundle {
    style: Style {
      size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
      flex_direction: FlexDirection::Column,
      ..default()
    },
    // background_color: Color::rgba(0.1, 0.1, 0.1, 0.6).into(),
    ..default()
  }).with_children(|root| {

    // FPS section
    root.spawn(NodeBundle {
      style: Style {
        padding: UiRect {
          top: Val::Px(5.0),
          left: Val::Px(5.0),
          ..default()
        },
        ..default()
      },
      ..default()
    }).with_children(|fps_root| {
      fps_root.spawn((TextBundle::from_sections([
        TextSection::new(
          "FPS: ",
          TextStyle {
            font: asset_server.load("fonts/Roboto-Regular.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
          }
        ),
        TextSection::from_style(TextStyle {
          font: asset_server.load("fonts/Roboto-Regular.ttf"),
          font_size: 20.0,
          color: Color::YELLOW,
        })
      ]), FpsText));
    });

    // TODO: other UI elements
  });

  // spawn polygons
}

fn print_fps(
  diagnostics: Res<Diagnostics>, 
  mut query: Query<&mut Text, With<FpsText>>
) {
  for mut text in &mut query {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(value) = fps.smoothed() {
        // Update the value of the second section
        text.sections[1].value = format!("{value:.2}");
        match value {
          a if a >= 59.0 => text.sections[1].style.color = Color::GREEN,
          a if a >= 36.0 => text.sections[1].style.color = Color::YELLOW,
          _ => text.sections[1].style.color = Color::RED,
        }
      }
    }
  }
}

fn update(
  time: Res<Time>, 
  mut timer: ResMut<Sec>,
) {
  if !timer.0.tick(time.delta()).just_finished() {
    return;
  }

  println!("do things (movement)");
}

fn end() {
  println!("end scene 2");
}