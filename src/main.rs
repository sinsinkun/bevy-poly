// prevent console on release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PresentMode;

// import other files
pub mod util;
mod menu;
mod movement;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Bevy Poly".into(),
        resolution: (800.0,600.0).into(),
        present_mode: PresentMode::AutoVsync,
        // resizable: false,
        ..default()
      }),
      ..default()
    }))
    .insert_resource(util::Sec(Timer::from_seconds(1.0, TimerMode::Repeating)))
    .add_state::<util::AppState>()
    .add_plugin(menu::MenuPlugin)
    .add_plugin(movement::MovementPlugin)
    .run();
}
