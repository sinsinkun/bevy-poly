use bevy::prelude::*;

use crate::util::{ AppState, Sec };

// ---- PLUGIN ----
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn name(&self) -> &str {
    "Movement Screen"
  }

  fn build(&self, app: &mut App) {
    app.add_system(start.in_schedule(OnEnter(AppState::Movement)))
      .add_system(update.run_if(in_state(AppState::Movement)))
      .add_system(end.in_schedule(OnExit(AppState::Movement)));
  }
}

// ---- SYSTEMS ----
fn start() {
  println!("start scene 2");
}

fn update(
  time: Res<Time>, 
  mut timer: ResMut<Sec>,
) {
  if !timer.0.tick(time.delta()).just_finished() {
    return;
  }

  println!("running scene 2");
}

fn end() {
  println!("end scene 2");
}